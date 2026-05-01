use std::{borrow::Cow, collections::HashMap, io::Cursor};

use flatgeobuf::FallibleStreamingIterator;
use serde::Serialize;

#[derive(Serialize)]
struct Feat {
    name: Cow<'static, str>,
    child: Child,
}

#[derive(Serialize)]
struct Child {
    value: i32,
    #[serde(skip, default = "geo_types::LineString::empty")]
    shape: geo_types::LineString,
}

#[test]
fn ser_test() -> anyhow::Result<()> {
    let fgb_writer = flatgeobuf::FgbWriter::create("", flatgeobuf::GeometryType::Unknown)?;
    let mut fgb_ser = geoserde::fgb::FeatureSerializer::new(fgb_writer);

    let feat = Feat {
        name: "hello".into(),
        child: Child {
            value: 42,
            shape: crate::testing::ls(1),
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
            ("child.value".to_string(), "42".to_string()),
        ])
    );

    assert!(fgb_iter.next()?.is_some());
    assert!(fgb_iter.next()?.is_none());
    Ok(())
}
