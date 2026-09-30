use std::io::Cursor;

use flatgeobuf::geozero::{ColumnValue, FeatureProcessor, GeozeroGeometry, PropertyProcessor};
use geo_types::Geometry;
use serde::Deserialize;

mod testing;

#[derive(Debug, Deserialize)]
struct NoProps {}

// --- Geometry type mismatch errors ---

/// A geometry type mismatch is reachable through `source()`.
#[test]
fn point_to_polygon_fails() {
    let mut fgb_buf = Vec::new();
    {
        let mut w = testing::fgb_writer(flatgeobuf::GeometryType::Point);
        Geometry::Point(testing::p(0)).process_geom(&mut w).unwrap();
        w.feature_end(0).unwrap();
        w.write(&mut fgb_buf).unwrap();
    }

    let mut fgb_de = geoserde_fgb::LayerDeserializer::new(Cursor::new(fgb_buf)).unwrap();

    let err = fgb_de
        .features::<geo_types::Polygon, NoProps>()
        .next()
        .unwrap()
        .unwrap_err();
    let source = std::error::Error::source(&err).unwrap();
    assert_eq!(source.to_string(), "expected Polygon, found Point");
}

// --- Iterator exhaustion ---

/// Iterating past the last feature returns None.
#[test]
fn empty_file_returns_none() -> anyhow::Result<()> {
    let mut fgb_buf = Vec::new();
    {
        let w = testing::fgb_writer(flatgeobuf::GeometryType::Point);
        w.write(&mut fgb_buf)?;
    }

    let mut fgb_de = geoserde_fgb::LayerDeserializer::new(Cursor::new(fgb_buf))?;

    let result = fgb_de.features::<geo_types::Point, NoProps>().next();
    assert!(result.is_none());
    Ok(())
}

/// After consuming all features, deserialize_feature returns Ok(None).
#[test]
fn exhausted_iterator_returns_none() -> anyhow::Result<()> {
    let mut fgb_buf = Vec::new();
    {
        let mut w = testing::fgb_writer(flatgeobuf::GeometryType::Point);
        Geometry::Point(testing::p(0)).process_geom(&mut w)?;
        w.feature_end(0)?;
        w.write(&mut fgb_buf)?;
    }

    let mut fgb_de = geoserde_fgb::LayerDeserializer::new(Cursor::new(fgb_buf))?;

    // Consume the only feature.
    let first = fgb_de.deserialize_feature::<geo_types::Point, NoProps>()?;
    assert!(first.is_some());

    // Next call should return None.
    let second = fgb_de.deserialize_feature::<geo_types::Point, NoProps>()?;
    assert!(second.is_none());
    Ok(())
}

// --- Property type mismatch ---

/// Deserializing an Int column into a String field should fail.
#[test]
fn property_type_mismatch() {
    let mut fgb_buf = Vec::new();
    {
        let mut w = testing::fgb_writer(flatgeobuf::GeometryType::Point);
        Geometry::Point(testing::p(0)).process_geom(&mut w).unwrap();
        w.property(0, "value", &ColumnValue::Int(42)).unwrap();
        w.feature_end(0).unwrap();
        w.write(&mut fgb_buf).unwrap();
    }

    #[derive(Debug, Deserialize)]
    struct WrongType {
        #[allow(dead_code)]
        value: Vec<i32>,
    }

    let mut fgb_de = geoserde_fgb::LayerDeserializer::new(Cursor::new(fgb_buf)).unwrap();

    let err = fgb_de
        .features::<geo_types::Point, WrongType>()
        .next()
        .unwrap()
        .unwrap_err();
    assert!(matches!(
        &err,
        geoserde_fgb::de::Error::Feature { column: Some(c), .. } if c == "value"
    ));
    assert_eq!(err.to_string(), "failed to deserialize column `value`");
}

/// A missing required field in the properties struct should fail.
#[test]
fn missing_required_field() {
    let mut fgb_buf = Vec::new();
    {
        let mut w = testing::fgb_writer(flatgeobuf::GeometryType::Point);
        Geometry::Point(testing::p(0)).process_geom(&mut w).unwrap();
        w.property(0, "other", &ColumnValue::String("x")).unwrap();
        w.feature_end(0).unwrap();
        w.write(&mut fgb_buf).unwrap();
    }

    #[derive(Debug, Deserialize)]
    struct Required {
        #[allow(dead_code)]
        name: String,
    }

    let mut fgb_de = geoserde_fgb::LayerDeserializer::new(Cursor::new(fgb_buf)).unwrap();

    let err = fgb_de
        .features::<geo_types::Point, Required>()
        .next()
        .unwrap()
        .unwrap_err();
    // The missing field is not tied to any column in the file.
    assert!(matches!(
        err,
        geoserde_fgb::de::Error::Feature { column: None, .. }
    ));
}

/// A feature with no properties can be deserialized into an empty struct.
#[test]
fn no_properties_into_empty_struct() -> anyhow::Result<()> {
    let mut fgb_buf = Vec::new();
    {
        let mut w = testing::fgb_writer(flatgeobuf::GeometryType::Point);
        Geometry::Point(testing::p(0)).process_geom(&mut w)?;
        w.feature_end(0)?;
        w.write(&mut fgb_buf)?;
    }

    let mut fgb_de = geoserde_fgb::LayerDeserializer::new(Cursor::new(fgb_buf))?;

    let result = fgb_de.deserialize_feature::<geo_types::Point, NoProps>()?;
    assert!(result.is_some());
    Ok(())
}

/// Optional fields should not error when missing.
#[test]
fn optional_field_missing_ok() -> anyhow::Result<()> {
    let mut fgb_buf = Vec::new();
    {
        let mut w = testing::fgb_writer(flatgeobuf::GeometryType::Point);
        Geometry::Point(testing::p(0)).process_geom(&mut w)?;
        w.property(0, "name", &ColumnValue::String("test"))?;
        w.feature_end(0)?;
        w.write(&mut fgb_buf)?;
    }

    #[derive(Debug, Deserialize)]
    struct OptionalProps {
        name: String,
        #[allow(dead_code)]
        extra: Option<String>,
    }

    let mut fgb_de = geoserde_fgb::LayerDeserializer::new(Cursor::new(fgb_buf))?;
    let (_, props) = fgb_de
        .features::<geo_types::Point, OptionalProps>()
        .next()
        .unwrap()?;
    assert_eq!(props.name, "test");
    assert!(props.extra.is_none());
    Ok(())
}
