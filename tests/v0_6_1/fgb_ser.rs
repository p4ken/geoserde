#![cfg(feature = "fgb")]

use std::{collections::HashMap, io::Cursor};

use flatgeobuf::FallibleStreamingIterator;
use serde::Serialize;

#[derive(Serialize)]
struct Root {
    parent: Parent,
}

#[derive(Serialize)]
struct Parent {
    text: &'static str,
}

const ROOT: Root = Root {
    parent: Parent { text: "hello" },
};

const POINT: geo_types::Point = geo_types::Point(geo_types::Coord { x: 1.0, y: 2.0 });

#[test]
fn properties_ser_test() -> anyhow::Result<()> {
    let fgb_writer = flatgeobuf::FgbWriter::create("", flatgeobuf::GeometryType::Unknown)?;
    let mut prop_ser = geoserde::v0_6_1::fgb::ser::PropertiesSerializer::new(fgb_writer);
    for _ in 0..2 {
        let flat_ser = geoserde::v0_6_1::ser::TableSerializer::new(&mut prop_ser);
        // TODO: "serialize_property" would be better?
        ROOT.serialize(flat_ser)?;
        flatgeobuf::geozero::FeatureProcessor::feature_end(prop_ser.mut_inner(), 0)?;
    }
    let fgb_writer = prop_ser.into_inner();

    let mut fgb_buf = Vec::new();
    fgb_writer.write(&mut fgb_buf)?;
    let mut fgb_iter = flatgeobuf::FgbReader::open(Cursor::new(fgb_buf))?.select_all()?;
    let fgb_feat = fgb_iter.next()?.unwrap();
    let props = flatgeobuf::geozero::FeatureProperties::properties(fgb_feat)?;
    let exp_props = HashMap::from([("parent.text".to_string(), "hello".to_string())]);
    assert_eq!(props, exp_props);

    assert!(fgb_iter.next()?.is_some());
    assert!(fgb_iter.next()?.is_none());
    Ok(())
}

#[test]
#[ignore]
fn geometries_ser_test() -> anyhow::Result<()> {
    let mut fgb_writer = flatgeobuf::FgbWriter::create("", flatgeobuf::GeometryType::Unknown)?;
    for _ in 0..2 {
        let geom_ser = geoserde::v0_6_1::fgb::ser::GeometrySerializer::new(&mut fgb_writer);
        geoserde::v0_6_1::SerializeGeometry::serialize_geometry(&POINT, geom_ser)?;
    }
    Ok(())
}
