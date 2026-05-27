#![cfg(feature = "fgb")]

use std::{collections::HashMap, io::Cursor};

use flatgeobuf::FallibleStreamingIterator;
use geo_traits::to_geo::{
    ToGeoLineString, ToGeoMultiLineString, ToGeoMultiPoint, ToGeoPoint, ToGeoPolygon,
};
use geo_traits::{GeometryTrait, GeometryType};
use serde::Serialize;

mod testing;

#[derive(Serialize)]
struct Feat {
    name: String,
    count: i32,
}

#[test]
fn point_test() -> anyhow::Result<()> {
    let fgb_writer = testing::fgb_writer(flatgeobuf::GeometryType::Point);
    let mut fgb_ser = geoserde::fgb::FeatureSerializer::new(fgb_writer);

    let feat = Feat {
        name: "a".into(),
        count: 0,
    };
    fgb_ser.serialize_feature(&testing::p(0), &feat)?;

    let mut fgb_buf = Vec::new();
    fgb_ser.into_inner().write(&mut fgb_buf)?;

    let mut fgb_iter = flatgeobuf::FgbReader::open(Cursor::new(fgb_buf))?.select_all()?;
    {
        let fgb_feat = fgb_iter.next()?.unwrap();
        let geom = fgb_feat.geometry_trait().unwrap().unwrap();
        let point = match geom.as_type() {
            GeometryType::Point(p) => p.to_point(),
            _ => panic!("expected Point"),
        };
        assert_eq!(point, testing::p(0));
    }
    assert!(fgb_iter.next()?.is_none());
    Ok(())
}

#[test]
fn line_string_test() -> anyhow::Result<()> {
    let fgb_writer = testing::fgb_writer(flatgeobuf::GeometryType::LineString);
    let mut fgb_ser = geoserde::fgb::FeatureSerializer::new(fgb_writer);

    let feat = Feat {
        name: "b".into(),
        count: 0,
    };
    fgb_ser.serialize_feature(&testing::ls(0), &feat)?;

    let mut fgb_buf = Vec::new();
    fgb_ser.into_inner().write(&mut fgb_buf)?;

    let mut fgb_iter = flatgeobuf::FgbReader::open(Cursor::new(fgb_buf))?.select_all()?;
    {
        let fgb_feat = fgb_iter.next()?.unwrap();
        let geom = fgb_feat.geometry_trait().unwrap().unwrap();
        let ls = match geom.as_type() {
            GeometryType::LineString(ls) => ls.to_line_string(),
            _ => panic!("expected LineString"),
        };
        assert_eq!(ls, testing::ls(0));
    }
    assert!(fgb_iter.next()?.is_none());
    Ok(())
}

#[test]
fn properties_test() -> anyhow::Result<()> {
    let fgb_writer = testing::fgb_writer(flatgeobuf::GeometryType::Point);
    let mut fgb_ser = geoserde::fgb::FeatureSerializer::new(fgb_writer);

    let feat = Feat {
        name: "alpha".into(),
        count: 3,
    };
    fgb_ser.serialize_feature(&testing::p(0), &feat)?;

    let mut fgb_buf = Vec::new();
    fgb_ser.into_inner().write(&mut fgb_buf)?;

    let mut fgb_iter = flatgeobuf::FgbReader::open(Cursor::new(fgb_buf))?.select_all()?;
    let fgb_feat = fgb_iter.next()?.unwrap();
    let props = flatgeobuf::geozero::FeatureProperties::properties(fgb_feat)?;
    assert_eq!(
        props,
        HashMap::from([
            ("name".to_string(), "alpha".to_string()),
            ("count".to_string(), "3".to_string()),
        ])
    );

    assert!(fgb_iter.next()?.is_none());
    Ok(())
}

#[test]
fn features_test() -> anyhow::Result<()> {
    let fgb_writer = testing::fgb_writer(flatgeobuf::GeometryType::Point);
    let mut fgb_ser = geoserde::fgb::FeatureSerializer::new(fgb_writer);

    fgb_ser.serialize_feature(
        &testing::p(0),
        &Feat {
            name: "first".into(),
            count: 1,
        },
    )?;
    fgb_ser.serialize_feature(
        &testing::p(1),
        &Feat {
            name: "second".into(),
            count: 2,
        },
    )?;

    let mut fgb_buf = Vec::new();
    fgb_ser.into_inner().write(&mut fgb_buf)?;

    let mut fgb_iter = flatgeobuf::FgbReader::open(Cursor::new(fgb_buf))?.select_all()?;

    let f1 = fgb_iter.next()?.unwrap();
    let p1 = flatgeobuf::geozero::FeatureProperties::properties(f1)?;
    assert_eq!(p1.len(), 2);
    assert_eq!(p1["name"], "first");
    assert_eq!(p1["count"], "1");

    let f2 = fgb_iter.next()?.unwrap();
    let p2 = flatgeobuf::geozero::FeatureProperties::properties(f2)?;
    assert_eq!(p2.len(), 2);
    assert_eq!(p2["name"], "second");
    assert_eq!(p2["count"], "2");

    assert!(fgb_iter.next()?.is_none());
    Ok(())
}

#[test]
fn polygon_test() -> anyhow::Result<()> {
    let fgb_writer = testing::fgb_writer(flatgeobuf::GeometryType::Polygon);
    let mut fgb_ser = geoserde::fgb::FeatureSerializer::new(fgb_writer);

    let feat = Feat {
        name: "c".into(),
        count: 0,
    };
    fgb_ser.serialize_feature(&testing::donut(0), &feat)?;

    let mut fgb_buf = Vec::new();
    fgb_ser.into_inner().write(&mut fgb_buf)?;

    let mut fgb_iter = flatgeobuf::FgbReader::open(Cursor::new(fgb_buf))?.select_all()?;
    {
        let fgb_feat = fgb_iter.next()?.unwrap();
        let geom = fgb_feat.geometry_trait().unwrap().unwrap();
        let poly = match geom.as_type() {
            GeometryType::Polygon(p) => p.to_polygon(),
            _ => panic!("expected Polygon"),
        };
        assert_eq!(poly, testing::donut(0));
    }
    assert!(fgb_iter.next()?.is_none());
    Ok(())
}

#[test]
fn primitive_test() -> anyhow::Result<()> {
    #[derive(Serialize)]
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
        v_str: &'static str,
    }

    let fgb_writer = testing::fgb_writer(flatgeobuf::GeometryType::Point);
    let mut fgb_ser = geoserde::fgb::FeatureSerializer::new(fgb_writer);

    fgb_ser.serialize_feature(
        &testing::p(0),
        &Prim {
            v_bool: true,
            v_i8: -1,
            v_i16: -200,
            v_i32: -30000,
            v_i64: -4000000000,
            v_u8: 255,
            v_u16: 60000,
            v_u32: 4000000000,
            v_u64: 10000000000,
            v_f32: 1.5,
            v_f64: 2.5,
            v_str: "hello",
        },
    )?;

    let mut fgb_buf = Vec::new();
    fgb_ser.into_inner().write(&mut fgb_buf)?;

    let mut fgb_iter = flatgeobuf::FgbReader::open(Cursor::new(fgb_buf))?.select_all()?;
    let fgb_feat = fgb_iter.next()?.unwrap();
    let props = flatgeobuf::geozero::FeatureProperties::properties(fgb_feat)?;
    assert_eq!(props["v_bool"], "true");
    assert_eq!(props["v_i8"], "-1");
    assert_eq!(props["v_i16"], "-200");
    assert_eq!(props["v_i32"], "-30000");
    assert_eq!(props["v_i64"], "-4000000000");
    assert_eq!(props["v_u8"], "255");
    assert_eq!(props["v_u16"], "60000");
    assert_eq!(props["v_u32"], "4000000000");
    assert_eq!(props["v_u64"], "10000000000");
    assert_eq!(props["v_f32"], "1.5");
    assert_eq!(props["v_f64"], "2.5");
    assert_eq!(props["v_str"], "hello");

    assert!(fgb_iter.next()?.is_none());
    Ok(())
}

#[test]
fn multi_point_test() -> anyhow::Result<()> {
    let fgb_writer = testing::fgb_writer(flatgeobuf::GeometryType::MultiPoint);
    let mut fgb_ser = geoserde::fgb::FeatureSerializer::new(fgb_writer);

    let feat = Feat {
        name: "d".into(),
        count: 0,
    };
    fgb_ser.serialize_feature(&testing::multi_p(0), &feat)?;

    let mut fgb_buf = Vec::new();
    fgb_ser.into_inner().write(&mut fgb_buf)?;

    let mut fgb_iter = flatgeobuf::FgbReader::open(Cursor::new(fgb_buf))?.select_all()?;
    {
        let fgb_feat = fgb_iter.next()?.unwrap();
        let geom = fgb_feat.geometry_trait().unwrap().unwrap();
        let mp = match geom.as_type() {
            GeometryType::MultiPoint(mp) => mp.to_multi_point(),
            _ => panic!("expected MultiPoint"),
        };
        assert_eq!(mp, testing::multi_p(0));
    }
    assert!(fgb_iter.next()?.is_none());
    Ok(())
}

#[test]
fn multi_line_string_test() -> anyhow::Result<()> {
    let fgb_writer = testing::fgb_writer(flatgeobuf::GeometryType::MultiLineString);
    let mut fgb_ser = geoserde::fgb::FeatureSerializer::new(fgb_writer);

    let feat = Feat {
        name: "e".into(),
        count: 0,
    };
    fgb_ser.serialize_feature(&testing::multi_ls(0), &feat)?;

    let mut fgb_buf = Vec::new();
    fgb_ser.into_inner().write(&mut fgb_buf)?;

    let mut fgb_iter = flatgeobuf::FgbReader::open(Cursor::new(fgb_buf))?.select_all()?;
    {
        let fgb_feat = fgb_iter.next()?.unwrap();
        let geom = fgb_feat.geometry_trait().unwrap().unwrap();
        let mls = match geom.as_type() {
            GeometryType::MultiLineString(mls) => mls.to_multi_line_string(),
            _ => panic!("expected MultiLineString"),
        };
        assert_eq!(mls, testing::multi_ls(0));
    }
    assert!(fgb_iter.next()?.is_none());
    Ok(())
}
