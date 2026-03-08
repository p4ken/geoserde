pub mod de;
pub mod fgb;
mod geo;
pub mod ser;

pub const POINT: &str = "geoserde::Point";
pub const LINE_STRING: &str = "geoserde::LineString";
pub const POLYGON: &str = "geoserde::Polygon";
pub const GEOMETRY: &str = "geoserde::Geometry";

pub fn serialize<S: serde::Serializer>(
    geom: impl SerializeGeometry,
    ser: S,
) -> Result<S::Ok, S::Error> {
    geom.serialize_geometry(ser)
}

pub fn deserialize<'a, D: serde::Deserializer<'a>, G: DeserializeGeometry>(
    de: D,
) -> Result<G, D::Error> {
    G::deserialize_geometry(de)
}

pub trait SerializeGeometry {
    fn serialize_geometry<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error>;
}

pub trait DeserializeGeometry: Sized {
    fn deserialize_geometry<'a, D: serde::Deserializer<'a>>(de: D) -> Result<Self, D::Error>;
}

/// Feature to deserialize a geometry with no properties
#[derive(Debug, serde::Deserialize)]
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

impl serde::Serialize for Point {
    fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        let mut sink = ser.serialize_struct(POINT, 4)?;
        serde::ser::SerializeStruct::serialize_field(&mut sink, "x", &self.x)?;
        serde::ser::SerializeStruct::serialize_field(&mut sink, "y", &self.y)?;
        serde::ser::SerializeStruct::serialize_field(&mut sink, "z", &self.z)?;
        serde::ser::SerializeStruct::serialize_field(&mut sink, "m", &self.m)?;
        serde::ser::SerializeStruct::end(sink)
    }
}

impl<'de> serde::Deserialize<'de> for Point {
    fn deserialize<D: serde::Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        serde::Deserializer::deserialize_struct(
            de,
            POINT,
            de::POINT_FIELDS,
            de::PointVisitor::new(),
        )
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

impl<'de, T: de::FromPointSeq> serde::Deserialize<'de> for LineString<T> {
    fn deserialize<D: serde::Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        de.deserialize_newtype_struct(LINE_STRING, de::LineStringVisitor::new())
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Polygon<T>(pub T);

impl<'de, T: de::FromLineStringSeq> serde::Deserialize<'de> for Polygon<T> {
    fn deserialize<D: serde::Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        de.deserialize_newtype_struct(POLYGON, de::PolygonVisitor::new())
    }
}
