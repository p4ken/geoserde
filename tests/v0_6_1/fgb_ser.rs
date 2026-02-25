#![cfg(feature = "flatgeobuf")]

use flatgeobuf::GeometryType;
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

    let fgb_writer = flatgeobuf::FgbWriter::create("", GeometryType::Unknown)?;
    let mut prop_ser = geoserde::v0_6_1::fgb::ser::PropertySerializer::new(fgb_writer);
    let mut flat_ser = geoserde::v0_6_1::ser::prop::FlatProperties::new(&mut prop_ser);
    root.serialize(flat_ser)?;

    let mut buf = Vec::new();
    prop_ser.into_inner().write(&mut buf)?;
    Ok(())
}
