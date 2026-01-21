use serde::de::{value::SeqDeserializer, IntoDeserializer};

use crate::v0_6_1::Point;

pub struct PointIter<'a> {
    fbs: flatgeobuf::Geometry<'a>,
    index: usize,
}

impl<'a> PointIter<'a> {
    pub fn new(fbs: flatgeobuf::Geometry<'a>) -> Self {
        Self { fbs, index: 0 }
    }
}

impl PointIter<'_> {
    fn xy(&self) -> Option<[f64; 2]> {
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
        match self.fbs.xy() {
            Some(xy) => xy.len() / 2,
            None => 0,
        }
    }
}

impl IntoDeserializer<'_> for PointIter<'_> {
    type Deserializer = SeqDeserializer<Self, serde::de::value::Error>;

    fn into_deserializer(self) -> Self::Deserializer {
        SeqDeserializer::new(self)
    }
}

// struct MultiPart {}

// struct GeometryCollection {}
