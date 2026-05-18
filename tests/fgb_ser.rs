#![cfg(feature = "fgb")]

use std::{collections::HashMap, io::Cursor};

use flatgeobuf::FallibleStreamingIterator;
use geo_traits::to_geo::{ToGeoLineString, ToGeoPoint};
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
    let fgb_writer = flatgeobuf::FgbWriter::create("", flatgeobuf::GeometryType::Point)?;
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
    let fgb_writer = flatgeobuf::FgbWriter::create("", flatgeobuf::GeometryType::LineString)?;
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
    let fgb_writer = flatgeobuf::FgbWriter::create("", flatgeobuf::GeometryType::Point)?;
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
    let mut fgb_opt = flatgeobuf::FgbWriterOptions::default();
    fgb_opt.write_index = false;
    let fgb_writer =
        flatgeobuf::FgbWriter::create_with_options("", flatgeobuf::GeometryType::Point, fgb_opt)?;
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
fn bool_test() -> anyhow::Result<()> {
    #[derive(Serialize)]
    struct FeatWithBool {
        active: bool,
    }

    let fgb_writer = flatgeobuf::FgbWriter::create("", flatgeobuf::GeometryType::Point)?;
    let mut fgb_ser = geoserde::fgb::FeatureSerializer::new(fgb_writer);

    fgb_ser.serialize_feature(&testing::p(0), &FeatWithBool { active: true })?;

    let mut fgb_buf = Vec::new();
    fgb_ser.into_inner().write(&mut fgb_buf)?;

    let mut fgb_iter = flatgeobuf::FgbReader::open(Cursor::new(fgb_buf))?.select_all()?;
    let fgb_feat = fgb_iter.next()?.unwrap();
    let props = flatgeobuf::geozero::FeatureProperties::properties(fgb_feat)?;
    assert_eq!(props.len(), 1);
    assert_eq!(props["active"], "true");

    assert!(fgb_iter.next()?.is_none());
    Ok(())
}
