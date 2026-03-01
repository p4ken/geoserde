#![cfg(feature = "flatgeobuf")]

use flatgeobuf::GeometryType;
use serde::Serialize;

#[test]
#[ignore]
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
    let prop_ser = geoserde::v0_6_1::fgb::ser::PropertiesSerializer::new(fgb_writer);
    let flat_ser = geoserde::v0_6_1::ser::prop::FlatProperties::new(prop_ser);
    let fgb_writer = root.serialize(flat_ser)?;

    let mut buf = Vec::new();
    fgb_writer.write(&mut buf)?;
    Ok(())
}
