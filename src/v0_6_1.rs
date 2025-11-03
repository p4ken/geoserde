use std::marker::PhantomData;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub mod fgb;

pub fn serialize<S: serde::Serializer>(
    geom: impl SerializeGeometry,
    ser: S,
) -> Result<S::Ok, S::Error> {
    __GeoSerdeGeometry(geom).serialize(ser)
}

pub fn deserialize<'a, D: serde::Deserializer<'a>, G: DeserializeGeometry>(
    de: D,
) -> Result<G, D::Error> {
    G::deserialize_geometry(de)
}

// pub fn deserialize<'de, D: serde::Deserializer<'de>, G: DeserializeGeometry + 'de>(
//     de: D,
// ) -> Result<G, D::Error> {
//     de.deserialize_newtype_struct("geoserde::Geometry", GeometryVisitor(PhantomData))
// }

pub trait SerializeGeometry: Serialize {}
impl SerializeGeometry for geo_types::Point {}
impl<T: SerializeGeometry> SerializeGeometry for &T {}

// TODO: sealed
pub trait DeserializeGeometry: DeserializeOwned {
    fn deserialize_geometry<'a, D: serde::Deserializer<'a>>(de: D) -> Result<Self, D::Error> {
        Ok(__GeoSerdeGeometry::deserialize(de)?.0)
    }
}
impl DeserializeGeometry for geo_types::Point {}
// FIXME: data formats directly depend on geo_types structures.
// e.g. "Polygon" must have "exterior" field dispite it is private
// pub trait DeserializeGeometry: Sized {
//     fn deserialize_geometry(src: impl GeometryTrait<T = f64>) -> Self;
// }

// Wrapper to tell "this is the geometry" for data formats.
#[derive(Serialize, Deserialize)]
struct __GeoSerdeGeometry<T>(T);

struct GeometryVisitor<G>(PhantomData<G>);
impl<'de, G: DeserializeGeometry + 'de> serde::de::Visitor<'de> for GeometryVisitor<G> {
    type Value = G;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("enum of geometry")
    }
    // fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    // where
    //     A: serde::de::MapAccess<'de>,
    // {
    //     // Error: This G must be concrete type which implements Deserialize
    //     let (k, v): (&'static str, G) = map.next_entry().unwrap().unwrap();
    //     todo!()
    // }
    fn visit_newtype_struct<D>(self, de: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        todo!()
    }
}
