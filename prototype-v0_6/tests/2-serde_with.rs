use std::marker::PhantomData;

use serde::{
    de::{DeserializeOwned, MapAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};

#[derive(Serialize, Deserialize)]
struct Child1 {
    #[serde(with = "geometry")]
    loc: geo_types::Point,
    count: i32,
}

#[derive(Serialize)]
struct MyFeature1 {
    child: Child1,
    title: String,
}

pub mod geometry {
    use geo_traits::{CoordTrait, PointTrait};
    use serde::{ser::SerializeStruct, Deserialize, Serialize};

    pub fn serialize<S: serde::Serializer>(
        geom: impl super::SerializeGeometry,
        ser: S,
    ) -> Result<S::Ok, S::Error> {
        super::__GeoSerdeGeometry(geom).serialize(ser)
    }
    pub fn deserialize<'a, D: serde::Deserializer<'a>, G: super::DeserializeGeometry>(
        de: D,
    ) -> Result<G, D::Error> {
        Ok(super::__GeoSerdeGeometry::deserialize(de)?.0)
    }
}

pub trait SerializeGeometry : Serialize {}
impl SerializeGeometry for geo_types::Point {}
impl<T: SerializeGeometry> SerializeGeometry for &T {}

// TODO: sealed
pub trait DeserializeGeometry : DeserializeOwned {}
impl DeserializeGeometry for geo_types::Point {}

// Wrapper to tell "this is the geometry" for data formats.
#[derive(Serialize, Deserialize)]
struct __GeoSerdeGeometry<T>(T);

fn main() {}
