#![cfg(feature = "geozero")]

use std::vec;

use geoserde::FeatureSerializer;
use serde::Serialize;

#[test]
#[should_panic]
fn no_geometry_test() {
    let feat = MyFeature { seq: vec![] };
    let mut sink = geozero::ProcessorSink;
    let mut sut = FeatureSerializer::new(&mut sink);
    feat.serialize(&mut sut).unwrap();
}

#[derive(Serialize)]
struct MyFeature {
    seq: Vec<f64>,
}
