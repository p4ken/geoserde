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
        let mut w = testing::fgb_writer(flatgeobuf::GeometryType::Point);
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
        let mut w = testing::fgb_writer(flatgeobuf::GeometryType::LineString);
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
        let mut w = testing::fgb_writer(flatgeobuf::GeometryType::Point);
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
        let mut w = testing::fgb_writer(flatgeobuf::GeometryType::Point);

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
fn polygon_test() -> anyhow::Result<()> {
    let mut fgb_buf = Vec::new();
    {
        let mut w = testing::fgb_writer(flatgeobuf::GeometryType::Polygon);
        Geometry::Polygon(testing::donut(0)).process_geom(&mut w)?;
        w.feature_end(0)?;
        w.write(&mut fgb_buf)?;
    }

    let fgb_reader = flatgeobuf::FgbReader::open(Cursor::new(fgb_buf))?;
    let mut fgb_de = geoserde::fgb::FeatureDeserializer::new(fgb_reader)?;

    #[derive(Deserialize)]
    struct NoProps {}
    let (geom, _) = fgb_de.deserialize_feature::<geo_types::Polygon, NoProps>()?;
    assert_eq!(geom, testing::donut(0));
    Ok(())
}

#[test]
fn primitive_test() -> anyhow::Result<()> {
    let mut fgb_buf = Vec::new();
    {
        let mut w = testing::fgb_writer(flatgeobuf::GeometryType::Point);
        Geometry::Point(testing::p(0)).process_geom(&mut w)?;
        w.property(0, "v_bool", &ColumnValue::Bool(true))?;
        w.property(1, "v_i8", &ColumnValue::Byte(-1))?;
        w.property(2, "v_i16", &ColumnValue::Short(-200))?;
        w.property(3, "v_i32", &ColumnValue::Int(-30000))?;
        w.property(4, "v_i64", &ColumnValue::Long(-4000000000))?;
        w.property(5, "v_u8", &ColumnValue::UByte(255))?;
        w.property(6, "v_u16", &ColumnValue::UShort(60000))?;
        w.property(7, "v_u32", &ColumnValue::UInt(4000000000))?;
        w.property(8, "v_u64", &ColumnValue::ULong(10000000000))?;
        w.property(9, "v_f32", &ColumnValue::Float(1.5))?;
        w.property(10, "v_f64", &ColumnValue::Double(2.5))?;
        w.property(11, "v_str", &ColumnValue::String("hello"))?;
        w.feature_end(0)?;
        w.write(&mut fgb_buf)?;
    }

    #[derive(Debug, Deserialize)]
    struct Prim {
        v_bool: bool,
        v_i8: i8,
        v_i16: i16,
        v_i32: i32,
        v_i64: i64,
        v_u8: u8,
        v_u16: u16,
        v_u32: u32,
        v_u64: u64,
        v_f32: f32,
        v_f64: f64,
        v_str: String,
    }

    let fgb_reader = flatgeobuf::FgbReader::open(Cursor::new(fgb_buf))?;
    let mut fgb_de = geoserde::fgb::FeatureDeserializer::new(fgb_reader)?;
    let (_, p) = fgb_de.deserialize_feature::<geo_types::Point, Prim>()?;
    assert!(p.v_bool);
    assert_eq!(p.v_i8, -1);
    assert_eq!(p.v_i16, -200);
    assert_eq!(p.v_i32, -30000);
    assert_eq!(p.v_i64, -4000000000);
    assert_eq!(p.v_u8, 255);
    assert_eq!(p.v_u16, 60000);
    assert_eq!(p.v_u32, 4000000000);
    assert_eq!(p.v_u64, 10000000000);
    assert_eq!(p.v_f32, 1.5);
    assert_eq!(p.v_f64, 2.5);
    assert_eq!(p.v_str, "hello");
    Ok(())
}
