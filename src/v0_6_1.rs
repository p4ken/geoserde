use serde::{Deserialize, Serialize};

mod de;
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
    G::deserialize_geometry(de)
}

pub trait SerializeGeometry: Serialize {}
impl SerializeGeometry for geo_types::Point {}
impl<T: SerializeGeometry> SerializeGeometry for &T {}

pub trait DeserializeGeometry: Sized {
    fn deserialize_geometry<'a, D: serde::Deserializer<'a>>(de: D) -> Result<Self, D::Error>;
}

/// Feature to deserialize a geometry with no properties
#[derive(Debug, Deserialize)]
pub struct GeometrySink<G: DeserializeGeometry> {
    #[serde(deserialize_with = "deserialize")]
    #[serde(rename = "geoserde::geometry")]
    /// Deserialized geometry
    pub g: G,
}

#[derive(Debug, Clone, Copy, Default, Deserialize)]
#[serde(rename = "geoserde::Point")]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: Option<f64>,
    pub m: Option<f64>,
}

#[derive(Debug, Clone, Copy, Default, Deserialize)]
#[serde(rename = "geoserde::LineString")]
pub struct LineString<T>(pub T);

#[derive(Debug, Clone, Copy, Default, Deserialize)]
#[serde(rename = "geoserde::Polygon")]
pub struct Polygon<T>(pub T);
