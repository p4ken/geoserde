use serde::de::{value::SeqDeserializer, IntoDeserializer};

use crate::v0_6_1::Point;

pub struct PointIter<'a> {
    fbs: flatgeobuf::Geometry<'a>,
    index: usize,
    len: usize,
}

impl<'a> PointIter<'a> {
    pub fn new(fbs: flatgeobuf::Geometry<'a>) -> Self {
        let len = fbs.xy().map(|xy| xy.len() / 2).unwrap_or(0);
        Self { fbs, index: 0, len }
    }

    pub fn with_range(fbs: flatgeobuf::Geometry<'a>, start: usize, end: usize) -> Self {
        Self {
            fbs,
            index: start,
            len: end,
        }
    }
}

impl PointIter<'_> {
    fn xy(&self) -> Option<[f64; 2]> {
        if self.index >= self.len {
            return None;
        }
        let mut iter = self.fbs.xy()?.iter().skip(self.index * 2);
        Some([iter.next()?, iter.next()?])
    }

    fn z(&self) -> Option<f64> {
        self.fbs.z()?.iter().nth(self.index)
    }

    fn m(&self) -> Option<f64> {
        self.fbs.m()?.iter().nth(self.index)
    }
}

impl Iterator for PointIter<'_> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let [x, y] = self.xy()?;
        let z = self.z();
        let m = self.m();
        self.index += 1;
        Some(Point { x, y, z, m })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }
}

impl ExactSizeIterator for PointIter<'_> {
    fn len(&self) -> usize {
        self.len.saturating_sub(self.index)
    }
}

impl IntoDeserializer<'_> for PointIter<'_> {
    type Deserializer = SeqDeserializer<Self, serde::de::value::Error>;

    fn into_deserializer(self) -> Self::Deserializer {
        SeqDeserializer::new(self)
    }
}

struct LineStringIter<'a> {
    fbs: flatgeobuf::Geometry<'a>,
    index: usize,
}

impl<'a> Iterator for LineStringIter<'a> {
    type Item = PointIter<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let ends = self.fbs.ends()?;

        if self.index >= ends.len() {
            return None;
        }

        let start = if self.index == 0 {
            0
        } else {
            ends.get(self.index - 1) as usize
        };
        let end = ends.get(self.index) as usize;

        self.index += 1;

        Some(PointIter::with_range(self.fbs, start, end))
    }
}

struct PolygonIter {}
