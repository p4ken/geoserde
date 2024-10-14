use geo_types::Point;
use serde::Serialize;

fn main() {
    let feat = MyFeature::default();
}

use geoserde::geometry;

#[derive(Default, Serialize)]
struct MyFeature {
    #[serde(with = "geometry")]
    loc: Point,
    title: String,
}

pub mod geoserde {
    pub use geometry::serialize;

    pub mod geometry {
        pub fn serialize<T, S>(t: &T, s: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            todo!()
        }
    }
}
