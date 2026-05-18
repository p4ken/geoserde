#![cfg(feature = "fgb")]

use std::io::Cursor;

use flatgeobuf::geozero::{ColumnValue, FeatureProcessor, GeozeroGeometry, PropertyProcessor};
use geo_types::Geometry;
use serde::Deserialize;

mod testing;

#[test]
fn point_test() -> anyhow::Result<()> {
    let mut fgb_buf = Vec::new();
    {
        let mut w = flatgeobuf::FgbWriter::create("", flatgeobuf::GeometryType::Point)?;
        Geometry::Point(testing::p(0)).process_geom(&mut w)?;
        w.feature_end(0)?;
        w.write(&mut fgb_buf)?;
    }

    let fgb_reader = flatgeobuf::FgbReader::open(Cursor::new(fgb_buf))?;
    let mut fgb_de = geoserde::fgb::FeatureDeserializer::new(fgb_reader)?;

    #[derive(Deserialize)]
    struct NoProps {}
    let (geom, _) = fgb_de.deserialize_feature::<geo_types::Point, NoProps>()?;
    assert_eq!(geom, testing::p(0));
    Ok(())
}

#[test]
fn line_string_test() -> anyhow::Result<()> {
    let mut fgb_buf = Vec::new();
    {
        let mut w = flatgeobuf::FgbWriter::create("", flatgeobuf::GeometryType::LineString)?;
        Geometry::LineString(testing::ls(0)).process_geom(&mut w)?;
        w.feature_end(0)?;
        w.write(&mut fgb_buf)?;
    }

    let fgb_reader = flatgeobuf::FgbReader::open(Cursor::new(fgb_buf))?;
    let mut fgb_de = geoserde::fgb::FeatureDeserializer::new(fgb_reader)?;

    #[derive(Deserialize)]
    struct NoProps {}
    let (geom, _) = fgb_de.deserialize_feature::<geo_types::LineString, NoProps>()?;
    assert_eq!(geom, testing::ls(0));
    Ok(())
}

#[test]
fn feature_test() -> anyhow::Result<()> {
    let mut fgb_buf = Vec::new();
    {
        let mut w = flatgeobuf::FgbWriter::create("", flatgeobuf::GeometryType::Point)?;
        Geometry::Point(testing::p(0)).process_geom(&mut w)?;
        w.property(0, "name", &ColumnValue::String("gamma"))?;
        w.property(1, "count", &ColumnValue::Int(5))?;
        w.feature_end(0)?;
        w.write(&mut fgb_buf)?;
    }

    #[derive(Debug, Deserialize)]
    struct Feat {
        name: String,
        count: i32,
    }

    let fgb_reader = flatgeobuf::FgbReader::open(Cursor::new(fgb_buf))?;
    let mut fgb_de = geoserde::fgb::FeatureDeserializer::new(fgb_reader)?;
    let (_, props) = fgb_de.deserialize_feature::<geo_types::Point, Feat>()?;
    assert_eq!(props.name, "gamma");
    assert_eq!(props.count, 5);
    Ok(())
}

#[test]
fn features_test() -> anyhow::Result<()> {
    let mut fgb_buf = Vec::new();
    {
        let mut w = flatgeobuf::FgbWriter::create("", flatgeobuf::GeometryType::Point)?;

        Geometry::Point(testing::p(0)).process_geom(&mut w)?;
        w.property(0, "seq", &ColumnValue::Int(1))?;
        w.feature_end(0)?;

        Geometry::Point(testing::p(1)).process_geom(&mut w)?;
        w.property(0, "seq", &ColumnValue::Int(2))?;
        w.feature_end(1)?;

        w.write(&mut fgb_buf)?;
    }

    #[derive(Debug, Deserialize)]
    struct Feat {
        seq: i32,
    }

    let fgb_reader = flatgeobuf::FgbReader::open(Cursor::new(fgb_buf))?;
    let mut fgb_de = geoserde::fgb::FeatureDeserializer::new(fgb_reader)?;

    let (geom1, feat1) = fgb_de.deserialize_feature::<geo_types::Point, Feat>()?;
    assert_eq!(geom1, testing::p(0));
    assert_eq!(feat1.seq, 1);

    let (geom2, feat2) = fgb_de.deserialize_feature::<geo_types::Point, Feat>()?;
    assert_eq!(geom2, testing::p(1));
    assert_eq!(feat2.seq, 2);
    Ok(())
}

#[test]
fn float_test() -> anyhow::Result<()> {
    let mut fgb_buf = Vec::new();
    {
        let mut w = flatgeobuf::FgbWriter::create("", flatgeobuf::GeometryType::Point)?;
        Geometry::Point(testing::p(0)).process_geom(&mut w)?;
        w.property(0, "ratio", &ColumnValue::Double(1.5))?;
        w.feature_end(0)?;
        w.write(&mut fgb_buf)?;
    }

    #[derive(Debug, Deserialize)]
    struct Feat {
        ratio: f64,
    }

    let fgb_reader = flatgeobuf::FgbReader::open(Cursor::new(fgb_buf))?;
    let mut fgb_de = geoserde::fgb::FeatureDeserializer::new(fgb_reader)?;
    let (_, props) = fgb_de.deserialize_feature::<geo_types::Point, Feat>()?;
    assert_eq!(props.ratio, 1.5);
    Ok(())
}
