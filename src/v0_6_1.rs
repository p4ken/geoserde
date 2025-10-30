use serde::{de::DeserializeOwned, Deserialize, Serialize};

mod fgb;

pub fn serialize<S: serde::Serializer>(
    geom: impl SerializeGeometry,
    ser: S,
) -> Result<S::Ok, S::Error> {
    __GeoSerdeGeometry(geom).serialize(ser)
}
pub fn deserialize<'a, D: serde::Deserializer<'a>, G: DeserializeGeometry>(
    de: D,
) -> Result<G, D::Error> {
    Ok(__GeoSerdeGeometry::deserialize(de)?.0)
}

pub trait SerializeGeometry: Serialize {}
impl SerializeGeometry for geo_types::Point {}
impl<T: SerializeGeometry> SerializeGeometry for &T {}

// TODO: sealed
pub trait DeserializeGeometry: DeserializeOwned {}
impl DeserializeGeometry for geo_types::Point {}

// Wrapper to tell "this is the geometry" for data formats.
#[derive(Serialize, Deserialize)]
struct __GeoSerdeGeometry<T>(T);
