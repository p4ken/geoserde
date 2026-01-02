use serde::{Deserialize, Serialize};

#[cfg(feature = "flatgeobuf")]
pub mod fgb;
mod geo;

pub fn serialize<S: serde::Serializer>(
    geom: impl SerializeGeometry,
    ser: S,
) -> Result<S::Ok, S::Error> {
    geom.serialize(ser)
}

pub fn deserialize<'a, D: serde::Deserializer<'a>, G: DeserializeGeometry>(
    de: D,
) -> Result<G, D::Error> {
    G::deserialize(de)
}

/// Feature to deserialize a geometry with no properties
#[derive(Debug, Deserialize)]
pub struct GeometrySink<G: DeserializeGeometry> {
    #[serde(deserialize_with = "deserialize")]
    #[serde(rename = "geoserde::geometry")]
    /// Deserialized geometry
    pub g: G,
}

pub trait SerializeGeometry: Serialize {}
impl SerializeGeometry for geo_types::Point {}
impl<T: SerializeGeometry> SerializeGeometry for &T {}

pub trait DeserializeGeometry: Sized {
    fn deserialize<'a, D: serde::Deserializer<'a>>(de: D) -> Result<Self, D::Error>;
}

#[derive(Deserialize)]
#[serde(rename = "geoserde::Point")]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: Option<f64>,
    pub m: Option<f64>,
}

// #[derive(Deserialize)]
// #[serde(rename = "geoserde::LineString")]
// struct LineString<I: Iterator<Item = Point>>(I);
