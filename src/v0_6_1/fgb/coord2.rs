use std::iter::{Skip, Take};

use geo_types::Coord;
use serde::de::{IntoDeserializer, SeqAccess};

use crate::v0_6_1::Point;

/// Iterates 1-demensional coordinate values in a `LineString` or a `MultiPoint`.
enum CoordIter<'a> {
    Empty,
    Range(Skip<Take<flatbuffers::VectorIter<'a, f64>>>),
    Full(flatbuffers::VectorIter<'a, f64>),
}

impl<'a> CoordIter<'a> {
    fn with_range(coords: Option<flatbuffers::Vector<'a, f64>>, start: usize, end: usize) -> Self {
        match coords {
            Some(vec) => Self::Range(vec.iter().take(end).skip(start)),
            None => Self::Empty,
        }
    }
    fn with_vec(coords: Option<flatbuffers::Vector<'a, f64>>) -> Self {
        match coords {
            Some(vec) => Self::Full(vec.iter()),
            None => Self::Empty,
        }
    }
}

impl Iterator for CoordIter<'_> {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            CoordIter::Empty => None,
            CoordIter::Range(range) => range.next(),
            CoordIter::Full(full) => full.next(),
        }
    }
}

/// Iterates coordinates in a `LineString` or a `MultiPoint`.
///
/// Coordinates are converted into [`geoserde::Point`].
pub struct PointIter<'a> {
    xy: CoordIter<'a>,
    z: CoordIter<'a>,
    m: CoordIter<'a>,
}

impl<'a> Iterator for PointIter<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.xy.next()?;
        let y = self.xy.next()?;
        let z = self.z.next();
        let m = self.m.next();
        Some(Point { x, y, z, m })
    }
}

/// Iterates LineStrings in a `Polygon` or a `MultiLineString`.
///
/// LineStrings are wrapped with [`PointIter`].
pub struct LineStringIter<'a> {
    start: usize,
    ends: Option<flatbuffers::VectorIter<'a, u32>>,
    xy: Option<flatbuffers::Vector<'a, f64>>,
    z: Option<flatbuffers::Vector<'a, f64>>,
    m: Option<flatbuffers::Vector<'a, f64>>,
}

impl<'a> Iterator for LineStringIter<'a> {
    type Item = PointIter<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.start, &mut self.ends) {
            (start, Some(ends)) => {
                let end = ends.next()? as usize;
                self.start = end;
                Some(PointIter {
                    xy: CoordIter::with_range(self.xy, start * 2, end * 2),
                    z: CoordIter::with_range(self.z, start, end),
                    m: CoordIter::with_range(self.m, start, end),
                })
            }
            (0, None) => {
                self.start = usize::MAX;
                Some(PointIter {
                    xy: CoordIter::with_vec(self.xy),
                    z: CoordIter::with_vec(self.z),
                    m: CoordIter::with_vec(self.m),
                })
            }
            (_, None) => None,
        }
    }
}

/// Iterates Polygons in a `MultiPolygon`.
///
/// Polygons are wrapped with [`LineStringIter`].
pub struct PolygonIter<'a> {
    parts: flatbuffers::VectorIter<'a, flatgeobuf::Geometry<'a>>,
}

impl<'a> Iterator for PolygonIter<'a> {
    type Item = LineStringIter<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let geom = self.parts.next()?;
        let ends = match geom.ends() {
            Some(ends) if ends.is_empty() => None,
            Some(ends) => Some(ends.iter()),
            None => None,
        };
        Some(LineStringIter {
            start: 0,
            ends: ends,
            xy: geom.xy(),
            z: geom.z(),
            m: geom.m(),
        })
    }
}

pub enum GeometryDeserializer<'a> {
    Point(PointIter<'a>),
    LineString(PointIter<'a>),
    Polygon(LineStringIter<'a>),
    MultiPoint(PointIter<'a>),
    MultiLineString(LineStringIter<'a>),
    MultiPolygon(PolygonIter<'a>),
}

impl<'de> GeometryDeserializer<'de> {
    pub fn new(fbs: flatgeobuf::Geometry<'de>, geom_type: flatgeobuf::GeometryType) -> Self {
        const UNKNOWN: flatgeobuf::GeometryType = flatgeobuf::GeometryType::Unknown;
        let geom_type = match fbs.type_() {
            flatgeobuf::GeometryType::Unknown => geom_type,
            t => t,
        };
        todo!()
    }
}

impl<'de> serde::de::EnumAccess<'de> for GeometryDeserializer<'de> {
    type Error = serde::de::value::Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        todo!()
    }
}

impl<'de> serde::de::VariantAccess<'de> for GeometryDeserializer<'de> {
    type Error = serde::de::value::Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        todo!()
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        todo!()
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }
}
