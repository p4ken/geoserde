use std::vec;

use geoserde::ser::FeatureSerializer;
use serde::Serialize;

#[cfg(feature = "geozero")]
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
