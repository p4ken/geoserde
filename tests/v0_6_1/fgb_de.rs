#![cfg(feature = "flatgeobuf")]

use std::io::Cursor;

use anyhow::Result;
use flatgeobuf::{FgbReader, GeometryType};
use geo_traits::to_geo::ToGeoGeometry;
use geoserde::v0_6_1::GeometrySink;
use geozero::{ColumnValue, FeatureProcessor, GeozeroGeometry, PropertyProcessor};
use serde::{de::DeserializeOwned, Deserialize};

use crate::testing;

#[test]
fn points_test() -> Result<()> {
    let mut fgb_buf = vec![];
    {
        let mut w = new_writer(GeometryType::Point);

        testing::p(0).to_geometry().process_geom(&mut w)?;
        w.property(0, "number", &ColumnValue::Int(1))?;
        w.feature_end(0)?;

        testing::p(1).to_geometry().process_geom(&mut w)?;
        w.property(0, "number", &ColumnValue::Int(2))?;
        w.feature_end(1)?;

        w.write(&mut fgb_buf)?;
    }

    let deserialized = deserialize_features::<GeometrySink<geo_types::Point>>(&fgb_buf)?;
    assert_eq!(deserialized[0].g, testing::p(0));
    assert_eq!(deserialized[1].g, testing::p(1));
    Ok(())
}

#[test]
fn properties_test() -> Result<()> {
    let mut fgb_buf = vec![];
    {
        let mut w = new_writer(GeometryType::LineString);

        w.property(0, "number", &ColumnValue::Int(1))?;
        w.property(1, "text", &ColumnValue::String("one"))?;
        w.feature_end(0)?;

        w.write(&mut fgb_buf)?;
    }

    #[derive(Debug, Deserialize)]
    struct MyFeature {
        number: i32,
        text: String,
    }

    let deserialized = deserialize_features::<MyFeature>(&fgb_buf)?;
    assert_eq!(deserialized[0].number, 1);
    assert_eq!(deserialized[0].text, "one");
    Ok(())
}

#[test]
fn line_string_with_property_test() -> Result<()> {
    let mut fgb_buf = vec![];
    {
        let mut w = new_writer(GeometryType::LineString);

        testing::ls(0).to_geometry().process_geom(&mut w)?;
        w.property(0, "number", &ColumnValue::Int(1))?;
        w.feature_end(0)?;

        w.write(&mut fgb_buf)?;
    }

    #[derive(Debug, Deserialize)]
    struct MyFeature {
        number: i32,
        #[serde(with = "geoserde::v0_6_1")]
        #[serde(rename = "geoserde::geometry")]
        geom: geo_types::LineString,
    }

    let deserialized = deserialize_features::<MyFeature>(&fgb_buf)?;
    assert_eq!(deserialized[0].number, 1);
    assert_eq!(deserialized[0].geom, testing::ls(0));
    Ok(())
}

#[test]
fn flatten_property_test() -> Result<()> {
    let mut fgb_buf = vec![];
    {
        let mut w = new_writer(GeometryType::Point);

        testing::p(0).to_geometry().process_geom(&mut w)?;
        w.property(0, "number", &ColumnValue::Int(1))?;
        w.feature_end(0)?;

        w.write(&mut fgb_buf)?;
    }

    #[derive(Debug, Deserialize)]
    struct Child {
        number: i32,
    }

    #[derive(Debug, Deserialize)]
    struct MyFeature {
        #[serde(flatten)]
        child: Child,
        #[serde(with = "geoserde::v0_6_1")]
        #[serde(rename = "geoserde::geometry")]
        geom: geo_types::Point,
    }

    let deserialized = deserialize_features::<MyFeature>(&fgb_buf)?;
    assert_eq!(deserialized[0].geom, testing::p(0));
    assert_eq!(deserialized[0].child.number, 1);
    Ok(())
}

// BUG: Virtual enum `Geometry` cannot deserialized with `flatten`.
#[ignore = "todo"]
#[test]
fn flatten_geometry_test() -> Result<()> {
    let mut fgb_buf = vec![];
    {
        let mut w = new_writer(GeometryType::Point);

        testing::p(0).to_geometry().process_geom(&mut w)?;
        w.property(0, "number", &ColumnValue::Int(1))?;
        w.feature_end(0)?;

        w.write(&mut fgb_buf)?;
    }

    #[derive(Debug, Deserialize)]
    struct Child {
        #[serde(with = "geoserde::v0_6_1")]
        #[serde(rename = "geoserde::geometry")]
        geom: geo_types::Point,
    }

    #[derive(Debug, Deserialize)]
    struct MyFeature {
        #[serde(flatten)]
        child: Child,
        number: i32,
    }

    let deserialized = deserialize_features::<MyFeature>(&fgb_buf)?;
    assert_eq!(deserialized[0].child.geom, testing::p(0));
    assert_eq!(deserialized[0].number, 1);
    Ok(())
}

#[test]
fn line_test() -> Result<()> {
    let mut fgb_buf = vec![];
    {
        let mut w = new_writer(GeometryType::LineString);

        testing::line(0).to_geometry().process_geom(&mut w)?;
        w.feature_end(0)?;

        w.write(&mut fgb_buf)?;
    }

    let deserialized = deserialize_features::<GeometrySink<geo_types::Line>>(&fgb_buf)?;
    assert_eq!(deserialized[0].g, testing::line(0));
    Ok(())
}

#[test]
fn rect_test() -> Result<()> {
    let mut fgb_buf = vec![];
    {
        let mut w = new_writer(GeometryType::Polygon);

        testing::rect(0).to_geometry().process_geom(&mut w)?;
        w.feature_end(0)?;

        w.write(&mut fgb_buf)?;
    }

    let deserialized = deserialize_features::<GeometrySink<geo_types::Rect>>(&fgb_buf)?;
    assert_eq!(deserialized[0].g, testing::rect(0));
    Ok(())
}

#[test]
fn triangle_test() -> Result<()> {
    let mut fgb_buf = vec![];
    {
        let mut w = new_writer(GeometryType::Polygon);

        testing::triangle(0).to_geometry().process_geom(&mut w)?;
        w.feature_end(0)?;

        w.write(&mut fgb_buf)?;
    }

    let deserialized = deserialize_features::<GeometrySink<geo_types::Triangle>>(&fgb_buf)?;
    assert_eq!(deserialized[0].g, testing::triangle(0));
    Ok(())
}

#[test]
fn polygon_test() -> Result<()> {
    let mut fgb_buf = vec![];
    {
        let mut w = new_writer(GeometryType::Polygon);

        testing::donut(0).to_geometry().process_geom(&mut w)?;
        w.feature_end(0)?;

        w.write(&mut fgb_buf)?;
    }

    let deserialized = deserialize_features::<GeometrySink<geo_types::Polygon>>(&fgb_buf)?;
    assert_eq!(deserialized[0].g, testing::donut(0));
    Ok(())
}

fn new_writer(geom_type: GeometryType) -> flatgeobuf::FgbWriter<'static> {
    let fgb_opt = flatgeobuf::FgbWriterOptions {
        write_index: false, // To keep the order of features
        ..Default::default()
    };
    flatgeobuf::FgbWriter::create_with_options("my_fgb", geom_type, fgb_opt).unwrap()
}

fn deserialize_features<T: DeserializeOwned>(fgb_buf: &[u8]) -> Result<Vec<T>> {
    let fgb_iter = FgbReader::open(Cursor::new(fgb_buf))?.select_all()?;
    let features = geoserde::v0_6_1::fgb::from_feature_iter(fgb_iter).collect::<Result<_, _>>()?;
    Ok(features)
}
