#![cfg(feature = "geozero")]

use geoserde::PropertySerializer;
use serde::Serialize;

#[test]
fn serialize_vec_property() {
    let v = vec![1, 3];
    let mut sink = geozero::ProcessorSink;
    let mut sut = PropertySerializer::new(0, "vec", &mut sink);
    let ret = v.serialize(&mut sut);
    assert!(ret.is_err());
}
