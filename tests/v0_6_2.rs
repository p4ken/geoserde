use std::{collections::HashMap, io::Cursor};

use flatgeobuf::FallibleStreamingIterator;
use serde::{Deserialize, Serialize};

const POINT: geo_types::Point = geo_types::Point(geo_types::Coord { x: 1.0, y: 2.0 });
const PROPS: Props = Props {
    name: "hello",
    value: 42,
};

#[derive(Serialize, Deserialize)]
struct Props {
    name: &'static str,
    value: i32,
}

#[test]
fn ser_test() -> anyhow::Result<()> {
    let fgb_writer = flatgeobuf::FgbWriter::create("", flatgeobuf::GeometryType::Unknown)?;
    let mut fgb_ser = geoserde::v0_6_2::fgb::ser::FeatureSerializer::new(fgb_writer);

    let geom = geo_types::Geometry::Point(POINT);
    fgb_ser.serialize_feature(&geom, &PROPS)?;
    fgb_ser.serialize_feature(&geom, &PROPS)?;

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

#[test]
fn de_test() -> anyhow::Result<()> {
    let mut fgb_buf = Vec::new();
    let mut fgb_writer = flatgeobuf::FgbWriter::create("", flatgeobuf::GeometryType::Unknown)?;
    flatgeobuf::geozero::GeozeroGeometry::process_geom(
        &geo_types::Geometry::Point(POINT),
        &mut fgb_writer,
    )?;
    flatgeobuf::geozero::PropertyProcessor::property(
        &mut fgb_writer,
        0,
        "value",
        &flatgeobuf::geozero::ColumnValue::Int(42),
    )?;
    fgb_writer.write(&mut fgb_buf)?;

    let fgb_reader = flatgeobuf::FgbReader::open(Cursor::new(fgb_buf))?;
    let mut fgb_de = geoserde::v0_6_2::fgb::de::FeatureDeserializer::new(fgb_reader);
    let (geom, props) = fgb_de.deserialize_feature::<geo_types::Point, Props>()?;
    Ok(())
}
