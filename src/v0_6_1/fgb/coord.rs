use serde::de::{value::SeqDeserializer, IntoDeserializer};

use crate::v0_6_1::Point;

pub struct PointIter<'de> {
    fbs_geom: flatgeobuf::Geometry<'de>,
    index: usize,
}

impl<'de> PointIter<'de> {
    pub fn new(fbs_geom: flatgeobuf::Geometry<'de>) -> Self {
        Self { fbs_geom, index: 0 }
    }
}

impl PointIter<'_> {
    fn xy(&self) -> Option<[f64; 2]> {
        let mut iter = self.fbs_geom.xy()?.iter().skip(self.index * 2);
        Some([iter.next()?, iter.next()?])
    }
    fn z(&self) -> Option<f64> {
        self.fbs_geom.z()?.iter().nth(self.index)
    }
    fn m(&self) -> Option<f64> {
        self.fbs_geom.m()?.iter().nth(self.index)
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
        match self.fbs_geom.xy() {
            Some(xy) => xy.len() / 2,
            None => 0,
        }
    }
}

impl<'de> IntoDeserializer<'de> for PointIter<'de> {
    type Deserializer = SeqDeserializer<Self, serde::de::value::Error>;

    fn into_deserializer(self) -> Self::Deserializer {
        SeqDeserializer::new(self)
    }
}

// struct MultiPart {}

// struct GeometryCollection {}
