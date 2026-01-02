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

#[derive(Debug, Clone, Copy, Default, Deserialize)]
#[serde(rename = "geoserde::Point")]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: Option<f64>,
    pub m: Option<f64>,
}

pub struct LineString<T>(pub T);
impl<'de, const N: usize> Deserialize<'de> for LineString<[Point; N]> {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor<const N: usize>;
        impl<'de, const N: usize> serde::de::Visitor<'de> for Visitor<N> {
            type Value = [Point; N];

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("sequence of Points")
            }

            fn visit_seq<S>(self, seq: S) -> Result<Self::Value, S::Error>
            where
                S: serde::de::SeqAccess<'de>,
            {
                visit_point_array(seq)
            }
        }

        de.deserialize_newtype_struct("geoserde::LineString", Visitor)
            .map(Self)
    }
}

fn visit_point_array<'de, S: serde::de::SeqAccess<'de>, const N: usize>(
    mut seq: S,
) -> Result<[Point; N], <S as serde::de::SeqAccess<'de>>::Error> {
    let mut array = [Point::default(); N];
    for i in 0..N {
        match seq.next_element()? {
            Some(p) => array[i] = p,
            None => return Err(serde::de::Error::invalid_length(i, &N.to_string().as_str())),
        }
    }
    Ok(array)
}
