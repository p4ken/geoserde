use std::io::Cursor;

use flatgeobuf::FallibleStreamingIterator;
use serde::Serialize;

mod testing;

#[derive(Serialize)]
struct Feat {
    name: String,
    count: i32,
}

#[test]
fn into_inner_test() -> anyhow::Result<()> {
    let mut ser = testing::layer_ser(geoserde_fgb::flatgeobuf::GeometryType::Point);

    let feat = Feat {
        name: "alpha".into(),
        count: 3,
    };
    ser.serialize_feature(testing::p(0), &feat)?;

    let mut fgb_buf = Vec::new();
    ser.into_inner().write(&mut fgb_buf)?;

    let mut fgb_iter = flatgeobuf::FgbReader::open(Cursor::new(fgb_buf))?.select_all()?;
    let fgb_feat = fgb_iter.next()?.unwrap();
    let props = flatgeobuf::geozero::FeatureProperties::properties(fgb_feat)?;
    assert_eq!(props["name"], "alpha");
    assert_eq!(props["count"], "3");

    assert!(fgb_iter.next()?.is_none());
    Ok(())
}

#[test]
fn options_test() -> anyhow::Result<()> {
    let options = geoserde_fgb::ser::LayerOptions::new()
        .name("layer")
        .geometry_type(geoserde_fgb::flatgeobuf::GeometryType::Point)
        .epsg(4326)
        .index(false)
        .title("t")
        .description("d")
        .metadata("m");
    let mut ser = geoserde_fgb::LayerSerializer::with_options(options)?;
    ser.serialize_feature(
        testing::p(0),
        &Feat {
            name: "a".into(),
            count: 0,
        },
    )?;
    let mut fgb_buf = Vec::new();
    ser.write(&mut fgb_buf)?;

    let fgb_reader = flatgeobuf::FgbReader::open(Cursor::new(fgb_buf))?;
    let header = fgb_reader.header();
    assert_eq!(header.name(), Some("layer"));
    assert_eq!(header.geometry_type(), flatgeobuf::GeometryType::Point);
    assert_eq!(header.crs().map(|crs| crs.code()), Some(4326));
    assert_eq!(header.index_node_size(), 0);
    assert_eq!(header.title(), Some("t"));
    assert_eq!(header.description(), Some("d"));
    assert_eq!(header.metadata(), Some("m"));
    Ok(())
}
