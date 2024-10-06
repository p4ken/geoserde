use geo_types::Point;
use serde::Serialize;

fn main() {
    let feat = MyFeature::default();
}

#[derive(Default, Serialize, Geoserde)]
struct MyFeature {
    #[geoserde(geometry)]
    loc: Point,
    title: String,
}
