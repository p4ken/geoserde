use serde::{Deserialize, Serialize};

mod de;
#[cfg(feature = "flatgeobuf")]
pub mod fgb;
mod geo;

pub const POINT: &str = "geoserde::Point";
pub const LINE_STRING: &str = "geoserde::LineString";
pub const GEOMETRY: &str = "geoserde::Geometry";

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

// #[derive(Debug, Clone, Deserialize)]
// #[serde(rename = "geoserde::Geometry")]
// enum Geometry<T: FromPointSeq> {
//     #[serde(rename = "geoserde::Point")]
//     Point(Point),
//     #[serde(rename = "geoserde::LineString")]
//     LineString(LineString<T>),
// }

/// Point representation to support new data structures or data formats.
/// Named as [`geoserde::POINT`](POINT) during (de)serialization.
///
/// **In most cases you should consider using [`geo_types::Point`] instead.**
#[derive(Debug, Clone, Copy, Default)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: Option<f64>,
    pub m: Option<f64>,
}

impl<'de> serde::Deserialize<'de> for Point {
    fn deserialize<D: serde::Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        de::deserialize_point(de)
    }
}

impl<'de> serde::de::IntoDeserializer<'de> for Point {
    type Deserializer = de::PointDeserializer;

    fn into_deserializer(self) -> Self::Deserializer {
        de::PointDeserializer::new(self)
    }
}

#[derive(Debug, Clone, Default)]
pub struct LineString<T>(pub T);

impl<'de, T: de::FromPointSeq> Deserialize<'de> for LineString<T> {
    fn deserialize<D: serde::Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        de.deserialize_newtype_struct(LINE_STRING, de::LineStringVisitor::new())
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Polygon<T>(pub T);
impl<'de, T: FromLineStringSeq> Deserialize<'de> for Polygon<T> {
    fn deserialize<D: serde::Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        struct LineStringSeqVisitor<T>(std::marker::PhantomData<T>);
        impl<'de, T: FromLineStringSeq> serde::de::Visitor<'de> for LineStringSeqVisitor<T> {
            type Value = T;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str(&"a sequence of geoserde::LineString")
            }
            fn visit_seq<A: serde::de::SeqAccess<'de>>(
                self,
                seq: A,
            ) -> Result<Self::Value, A::Error> {
                T::from_linestring_seq(seq)
            }
        }

        struct NewtypeVisitor<T>(std::marker::PhantomData<T>);
        impl<'de, T: FromLineStringSeq> serde::de::Visitor<'de> for NewtypeVisitor<T> {
            type Value = T;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("a newtype struct")
            }
            fn visit_newtype_struct<D: serde::Deserializer<'de>>(
                self,
                de: D,
            ) -> Result<Self::Value, D::Error> {
                de.deserialize_seq(LineStringSeqVisitor(std::marker::PhantomData))
            }
        }
        de.deserialize_newtype_struct(
            "geoserde::Polygon",
            NewtypeVisitor(std::marker::PhantomData),
        )
        .map(Self)
    }
}

pub trait FromLineStringSeq: Sized {
    fn from_linestring_seq<'de, A: serde::de::SeqAccess<'de>>(seq: A) -> Result<Self, A::Error>;
}
