#![cfg(feature = "flatgeobuf")]

use std::{collections::HashMap, io::Cursor};

use flatgeobuf::FallibleStreamingIterator;
use serde::Serialize;

#[test]
fn properties_ser_test() -> anyhow::Result<()> {
    #[derive(Serialize)]
    struct Root {
        parent: Parent,
    }

    #[derive(Serialize)]
    struct Parent {
        text: &'static str,
    }

    let root = Root {
        parent: Parent { text: "hello" },
    };

    let fgb_writer = flatgeobuf::FgbWriter::create("", flatgeobuf::GeometryType::Unknown)?;
    let prop_ser = geoserde::v0_6_1::fgb::ser::PropertiesSerializer::new(fgb_writer);
    let flat_ser = geoserde::v0_6_1::ser::prop::FlattenSerializer::new(prop_ser);
    let mut fgb_writer = root.serialize(flat_ser)?;
    flatgeobuf::geozero::FeatureProcessor::feature_end(&mut fgb_writer, 0)?;

    let mut fgb_buf = Vec::new();
    fgb_writer.write(&mut fgb_buf)?;
    let mut fgb_iter = flatgeobuf::FgbReader::open(Cursor::new(fgb_buf))?.select_all()?;
    let fgb_feat = fgb_iter.next()?.unwrap();
    let props = flatgeobuf::geozero::FeatureProperties::properties(fgb_feat)?;
    let exp_props = HashMap::from([("parent.text".to_string(), "hello".to_string())]);
    assert_eq!(props, exp_props);
    Ok(())
}
