use std::marker::PhantomData;

use serde::{ Deserializer, de::{EnumAccess, SeqAccess, VariantAccess, Visitor}};

use crate::v0_6_1::{LINE_STRING, LineString, POLYGON, Point};

/// Visitor for LineString::deserialize implementation.
pub struct LineStringVisitor<T>(PhantomData<T>);

impl<T> LineStringVisitor<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<'de, T: FromPointSeq> Visitor<'de> for LineStringVisitor<T> {
    type Value = LineString<T>;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(LINE_STRING)
    }

    /// Accept Geometry enum.
    ///
    /// Recommended for self-describing formats, which most GIS formats are.
    fn visit_enum<A: EnumAccess<'de>>(self, geometry: A) -> Result<Self::Value, A::Error>{
        match geometry.variant()? {
            // Extract Polygon from Geometry enum.
            // Expects LineString::deserialize called recursively and then Self::visitd_seq called.
            (LINE_STRING, line_string) => line_string.newtype_variant(),
            // Try flatten single ring of Polygon.
            // FIXME: Test inner ring raises error
            (POLYGON, polygon) => polygon.newtype_variant(),
            (name, _) => Err(serde::de::Error::unknown_variant(name, &[LINE_STRING])),
        }
    }

    /// Accept newtype and flatten inner sequence.
    fn visit_newtype_struct<D: Deserializer<'de>>(self, de: D) -> Result<Self::Value, D::Error> {
        de.deserialize_seq(self)
    }

    /// Accept sequence of Points.
    fn visit_seq<A: SeqAccess<'de>>(self, seq: A) -> Result<Self::Value, A::Error> {
        T::from_point_seq(seq).map(LineString)
    }
}

pub trait FromPointSeq : Sized {
    fn from_point_seq<'de, A: serde::de::SeqAccess<'de> >(seq: A) -> Result<Self, A::Error>;
}

impl<P: From<Point>> FromPointSeq for Vec<P> {
    fn from_point_seq<'de, A: serde::de::SeqAccess<'de>>( mut seq: A) -> Result<Self, A::Error> {
        // Optimize heap allocation
        let mut vec = Vec::with_capacity(seq.size_hint().unwrap_or(0));

        // Deserialize Point
        while let Some(point) = seq.next_element()? {
            vec.push(P::from(point));
        }
        Ok(vec)
    }
}

impl<P: From<Point>, const N: usize> FromPointSeq for [P; N] {
    fn from_point_seq<'de, A: serde::de::SeqAccess<'de>>( mut seq: A) -> Result<Self, A::Error> {
        if let Some(size) = seq.size_hint() && size < N {
            return Err(serde::de::Error::invalid_length(size, &N.to_string().as_str()));
        }

        let mut array = [Point::default(); N];
        let mut i = 0;
        // Deserialize Point
        while let Some(point) = seq.next_element()? {
            if i == N {
                // Ignore remaining elements
                break;
            }
            array[i] = point;
            i += 1;
        }
        if i < N {
            return Err(serde::de::Error::invalid_length(i, &N.to_string().as_str()));
        }

        Ok(array.map(Into::into))
    }
}
