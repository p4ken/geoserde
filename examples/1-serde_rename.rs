use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Child1 {
    #[serde(rename = "geometry")]
    loc: geo_types::Point,
    count: i32,
}

#[derive(Serialize)]
pub struct MyFeature1 {
    #[serde(flatten)]
    child: Child1,
    title: String,
}

fn main() {}
