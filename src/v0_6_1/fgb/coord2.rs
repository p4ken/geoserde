use std::iter::{Skip, Take};

use serde::de::{
    value::{Error, SeqDeserializer},
    IntoDeserializer,
};

use crate::v0_6_1::Point;

/// Iterates 1-demensional coordinate values in a `LineString` or a `MultiPoint`.
pub enum CoordIter<'a> {
    Empty,
    Full(flatbuffers::VectorIter<'a, f64>),
    Range(Skip<Take<flatbuffers::VectorIter<'a, f64>>>),
}

impl<'a> CoordIter<'a> {
    fn new(coords: Option<flatbuffers::Vector<'a, f64>>) -> Self {
        match coords {
            Some(vec) => Self::Full(vec.iter()),
            None => Self::Empty,
        }
    }
    fn with_range(coords: Option<flatbuffers::Vector<'a, f64>>, start: usize, end: usize) -> Self {
        match coords {
            Some(vec) => Self::Range(vec.iter().take(end).skip(start)),
            None => Self::Empty,
        }
    }
}

impl<'a> Into<CoordIter<'a>> for Option<flatbuffers::Vector<'a, f64>> {
    fn into(self) -> CoordIter<'a> {
        CoordIter::new(self)
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

impl<'a> PointIter<'a> {
    pub fn new(
        xy: impl Into<CoordIter<'a>>,
        z: impl Into<CoordIter<'a>>,
        m: impl Into<CoordIter<'a>>,
    ) -> Self {
        Self {
            xy: xy.into(),
            z: z.into(),
            m: m.into(),
        }
    }
}

impl<'a> Iterator for PointIter<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        Some(Point {
            x: self.xy.next()?,
            y: self.xy.next()?,
            z: self.z.next(),
            m: self.m.next(),
        })
    }
}

impl<'de> IntoDeserializer<'de> for PointIter<'de> {
    type Deserializer = SeqDeserializer<Self, Error>;

    fn into_deserializer(self) -> Self::Deserializer {
        SeqDeserializer::new(self)
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

impl<'a> LineStringIter<'a> {
    pub fn new(
        ends: flatbuffers::Vector<'a, u32>,
        xy: Option<flatbuffers::Vector<'a, f64>>,
        z: Option<flatbuffers::Vector<'a, f64>>,
        m: Option<flatbuffers::Vector<'a, f64>>,
    ) -> Self {
        Self {
            start: 0,
            ends: Some(ends.iter()),
            xy,
            z,
            m,
        }
    }
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
                Some(PointIter::new(self.xy, self.z, self.m))
            }
            (_, None) => None,
        }
    }
}

impl<'de> IntoDeserializer<'de> for LineStringIter<'de> {
    type Deserializer = SeqDeserializer<Self, Error>;

    fn into_deserializer(self) -> Self::Deserializer {
        SeqDeserializer::new(self)
    }
}

/// Iterates Polygons in a `MultiPolygon`.
///
/// Polygons are wrapped with [`LineStringIter`].
pub struct PolygonIter<'a> {
    parts: flatbuffers::VectorIter<'a, flatbuffers::ForwardsUOffset<flatgeobuf::Geometry<'a>>>,
}

impl<'a> PolygonIter<'a> {
    pub fn new(
        parts: flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<flatgeobuf::Geometry<'a>>>,
    ) -> Self {
        Self {
            parts: parts.iter(),
        }
    }
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

impl<'de> IntoDeserializer<'de> for PolygonIter<'de> {
    type Deserializer = SeqDeserializer<Self, Error>;

    fn into_deserializer(self) -> Self::Deserializer {
        SeqDeserializer::new(self)
    }
}
