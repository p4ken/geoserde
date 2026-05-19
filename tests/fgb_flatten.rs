use std::{borrow::Cow, collections::HashMap, io::Cursor};

use flatgeobuf::FallibleStreamingIterator;
use serde::{Deserialize, Serialize};

mod testing;

#[derive(Serialize, Deserialize)]
struct Feat {
    name: Cow<'static, str>,
    #[serde(flatten)]
    child: Child,
}

#[derive(Serialize, Deserialize)]
struct Child {
    value: i32,
    #[serde(skip, default = "geo_types::LineString::empty")]
    shape: geo_types::LineString,
}

#[test]
fn ser_test() -> anyhow::Result<()> {
    let fgb_writer = testing::fgb_writer(flatgeobuf::GeometryType::Unknown);
    let mut fgb_ser = geoserde::fgb::FeatureSerializer::new(fgb_writer);

    let feat = Feat {
        name: "hello".into(),
        child: Child {
            value: 42,
            shape: testing::ls(1),
        },
    };
    fgb_ser.serialize_feature(&feat.child.shape, &feat)?;
    fgb_ser.serialize_feature(&feat.child.shape, &feat)?;

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
    let mut fgb_writer = testing::fgb_writer(flatgeobuf::GeometryType::LineString);
    flatgeobuf::geozero::GeozeroGeometry::process_geom(
        &geo_types::Geometry::LineString(testing::ls(1)),
        &mut fgb_writer,
    )?;
    flatgeobuf::geozero::PropertyProcessor::property(
        &mut fgb_writer,
        0,
        "name",
        &flatgeobuf::geozero::ColumnValue::String("hello"),
    )?;
    flatgeobuf::geozero::PropertyProcessor::property(
        &mut fgb_writer,
        1,
        "value",
        &flatgeobuf::geozero::ColumnValue::Int(42),
    )?;
    flatgeobuf::geozero::FeatureProcessor::feature_end(&mut fgb_writer, 0)?;
    fgb_writer.write(&mut fgb_buf)?;

    let fgb_reader = flatgeobuf::FgbReader::open(Cursor::new(fgb_buf))?;
    let mut fgb_de = geoserde::fgb::FeatureDeserializer::new(fgb_reader)?;
    let (geom, props) = fgb_de.deserialize_feature::<geo_types::LineString, Feat>()?;

    assert_eq!(geom, testing::ls(1));
    assert_eq!(props.name, "hello");
    assert_eq!(props.child.value, 42);
    Ok(())
}
