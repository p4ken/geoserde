use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Child1 {
    #[serde(with = "geometry")]
    loc: geo_types::Point,
    count: i32,
}

#[derive(Serialize)]
pub struct MyFeature1 {
    child: Child1,
    title: String,
}

pub mod geometry {
    pub fn serialize<S: serde::Serializer>(
        point: &geo_types::Point,
        ser: S,
    ) -> Result<S::Ok, S::Error> {
        // 普通のフィールド名と被らない名前
        ser.serialize_newtype_struct("__geoserde_geometry", point)
    }
    pub fn deserialize<'a, D: serde::Deserializer<'a>, G>(_de: D) -> Result<G, D::Error> {
        // de -> reader // これができない
        // reader -> LineString etc.
        todo!()
    }
}

fn main() {}
