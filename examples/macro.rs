use geo_types::Point;
use serde::Serialize;

fn main() {}

pub mod geoserde {
    pub mod geometry {
        pub fn serialize<T, S: serde::Serializer>(_t: &T, _s: S) -> Result<S::Ok, S::Error> {
            todo!()
        }
    }
}

#[derive(Default, Serialize)]
struct MyFeature1 {
    #[serde(with = "geoserde::geometry")]
    loc: Point,
    title: String,
}

#[derive(Default, Serialize)]
struct MyFeature2 {
    // 技術的にこれだけで可能なのか？
    #[geoserde_(geometry)]
    loc: Point,
    title: String,
}
