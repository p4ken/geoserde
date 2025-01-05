use geoserde::Feature;
use serde::Serialize;

#[derive(Serialize, Feature)]
pub struct Child1 {
    #[serde(with = "geoserde::geometry")]
    loc: geo_types::Point,
    count: i32,
}

#[derive(Serialize)]
pub struct MyFeature1 {
    child: Child1,
    title: String,
}

fn main() {}
