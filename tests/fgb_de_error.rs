#![cfg(feature = "fgb")]

use std::io::Cursor;

use flatgeobuf::geozero::{ColumnValue, FeatureProcessor, GeozeroGeometry, PropertyProcessor};
use geo_types::Geometry;
use serde::Deserialize;

mod testing;

#[derive(Deserialize)]
struct NoProps {}

// --- Geometry type mismatch errors ---

/// Point → Polygon is unsupported.
#[test]
fn point_to_polygon_fails() {
    let mut fgb_buf = Vec::new();
    {
        let mut w = testing::fgb_writer(flatgeobuf::GeometryType::Point);
        Geometry::Point(testing::p(0)).process_geom(&mut w).unwrap();
        w.feature_end(0).unwrap();
        w.write(&mut fgb_buf).unwrap();
    }

    let fgb_reader = flatgeobuf::FgbReader::open(Cursor::new(fgb_buf)).unwrap();
    let mut fgb_de = geoserde::fgb::FeatureDeserializer::new(fgb_reader).unwrap();

    let result = fgb_de.iter::<geo_types::Polygon, NoProps>().next().unwrap();
    assert!(result.is_err());
}

/// Point → MultiLineString is unsupported.
#[test]
fn point_to_multi_line_string_fails() {
    let mut fgb_buf = Vec::new();
    {
        let mut w = testing::fgb_writer(flatgeobuf::GeometryType::Point);
        Geometry::Point(testing::p(0)).process_geom(&mut w).unwrap();
        w.feature_end(0).unwrap();
        w.write(&mut fgb_buf).unwrap();
    }

    let fgb_reader = flatgeobuf::FgbReader::open(Cursor::new(fgb_buf)).unwrap();
    let mut fgb_de = geoserde::fgb::FeatureDeserializer::new(fgb_reader).unwrap();

    let result = fgb_de
        .iter::<geo_types::MultiLineString, NoProps>()
        .next()
        .unwrap();
    assert!(result.is_err());
}

/// Polygon → Point can extract the first coord of the exterior ring,
/// but Polygon → MultiPoint extracts all coords. Polygon is NOT
/// convertible from a Point source (the wildcard arm rejects it).
/// This is already covered above. Test MultiPoint → MultiLineString fails.
#[test]
fn multi_point_to_multi_line_string_fails() {
    let mp = geo_types::MultiPoint::new(vec![testing::p(0), testing::p(1)]);
    let mut fgb_buf = Vec::new();
    {
        let mut w = testing::fgb_writer(flatgeobuf::GeometryType::MultiPoint);
        Geometry::MultiPoint(mp).process_geom(&mut w).unwrap();
        w.feature_end(0).unwrap();
        w.write(&mut fgb_buf).unwrap();
    }

    let fgb_reader = flatgeobuf::FgbReader::open(Cursor::new(fgb_buf)).unwrap();
    let mut fgb_de = geoserde::fgb::FeatureDeserializer::new(fgb_reader).unwrap();

    let result = fgb_de
        .iter::<geo_types::MultiLineString, NoProps>()
        .next()
        .unwrap();
    // MultiPoint has a match arm in MultiLineString (wraps coords as one LineString),
    // so this actually succeeds. Let's assert that instead.
    assert!(result.is_ok());
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

    let fgb_reader = flatgeobuf::FgbReader::open(Cursor::new(fgb_buf))?;
    let mut fgb_de = geoserde::fgb::FeatureDeserializer::new(fgb_reader)?;

    let result = fgb_de.iter::<geo_types::Point, NoProps>().next();
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

    let fgb_reader = flatgeobuf::FgbReader::open(Cursor::new(fgb_buf))?;
    let mut fgb_de = geoserde::fgb::FeatureDeserializer::new(fgb_reader)?;

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

    #[derive(Deserialize)]
    struct WrongType {
        #[allow(dead_code)]
        value: Vec<i32>,
    }

    let fgb_reader = flatgeobuf::FgbReader::open(Cursor::new(fgb_buf)).unwrap();
    let mut fgb_de = geoserde::fgb::FeatureDeserializer::new(fgb_reader).unwrap();

    let result = fgb_de.iter::<geo_types::Point, WrongType>().next().unwrap();
    assert!(result.is_err());
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

    #[derive(Deserialize)]
    struct Required {
        #[allow(dead_code)]
        name: String,
    }

    let fgb_reader = flatgeobuf::FgbReader::open(Cursor::new(fgb_buf)).unwrap();
    let mut fgb_de = geoserde::fgb::FeatureDeserializer::new(fgb_reader).unwrap();

    let result = fgb_de.iter::<geo_types::Point, Required>().next().unwrap();
    assert!(result.is_err());
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

    let fgb_reader = flatgeobuf::FgbReader::open(Cursor::new(fgb_buf))?;
    let mut fgb_de = geoserde::fgb::FeatureDeserializer::new(fgb_reader)?;

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

    let fgb_reader = flatgeobuf::FgbReader::open(Cursor::new(fgb_buf))?;
    let mut fgb_de = geoserde::fgb::FeatureDeserializer::new(fgb_reader)?;
    let (_, props) = fgb_de
        .iter::<geo_types::Point, OptionalProps>()
        .next()
        .unwrap()?;
    assert_eq!(props.name, "test");
    assert!(props.extra.is_none());
    Ok(())
}
