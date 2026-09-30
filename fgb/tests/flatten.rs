use std::{borrow::Cow, io::Cursor};

use serde::Deserialize;

mod testing;

#[derive(Deserialize)]
struct Feat {
    name: Cow<'static, str>,
    #[serde(flatten)]
    child: Child,
}

#[derive(Deserialize)]
struct Child {
    value: i32,
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

    let mut de = geoserde_fgb::LayerDeserializer::new(Cursor::new(fgb_buf))?;
    let (geom, props) = de
        .features::<geo_types::LineString, Feat>()
        .next()
        .unwrap()?;

    assert_eq!(geom, testing::ls(1));
    assert_eq!(props.name, "hello");
    assert_eq!(props.child.value, 42);
    Ok(())
}
