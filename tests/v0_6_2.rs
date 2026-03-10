use std::{collections::HashMap, io::Cursor};

use flatgeobuf::FallibleStreamingIterator;
use serde::Serialize;

#[derive(Serialize)]
struct Props {
    name: &'static str,
    value: i32,
}

#[test]
fn ser_test() -> anyhow::Result<()> {
    let fgb_writer = flatgeobuf::FgbWriter::create("", flatgeobuf::GeometryType::Unknown)?;
    let mut fgb_ser = geoserde::v0_6_2::fgb::ser::FeatureSerializer::new(fgb_writer);

    let point = geo_types::point!(x: 1.0_f64, y: 2.0);
    let geom = geo_types::Geometry::Point(point);
    let props = Props {
        name: "hello",
        value: 42,
    };
    fgb_ser.serialize_feature(&geom, &props)?;
    fgb_ser.serialize_feature(&geom, &props)?;

    let mut fgb_buf = Vec::new();
    fgb_ser.into_inner().write(&mut fgb_buf)?;

    // Read back and verify
    let mut fgb_iter = flatgeobuf::FgbReader::open(Cursor::new(fgb_buf))?.select_all()?;

    let fgb_feat = fgb_iter.next()?.unwrap();
    let feat_props = flatgeobuf::geozero::FeatureProperties::properties(fgb_feat)?;
    assert_eq!(
        feat_props,
        HashMap::from([
            ("name".to_string(), "hello".to_string()),
            ("value".to_string(), "42".to_string()),
        ])
    );

    assert!(fgb_iter.next()?.is_some());
    assert!(fgb_iter.next()?.is_none());
    Ok(())
}
