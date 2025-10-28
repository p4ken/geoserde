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
        point: &impl Into<geo_types::Geometry>,
        ser: S,
    ) -> Result<S::Ok, S::Error> {
        // 普通の構造体名と被らない名前
        let geom: geo_types::Geometry = point.into();
        ser.serialize_newtype_struct("__geoserde_geometry", geom)
    }
    pub fn deserialize<'a, D: serde::Deserializer<'a>, G>(_de: D) -> Result<G, D::Error> {
        // de -> reader // これができない
        // reader -> LineString etc.
        todo!()
    }
}

fn main() {}
