use std::marker::PhantomData;

use serde::{
    de::{EnumAccess, Error, SeqAccess, VariantAccess, Visitor},
    Deserializer,
};

use crate::v0_6_1::{Polygon, POLYGON};

/// Visitor for Polygon::deserialize implementation.
pub struct PolygonVisitor<T>(PhantomData<T>);

impl<T> PolygonVisitor<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<'de, T: FromLineStringSeq> Visitor<'de> for PolygonVisitor<T> {
    type Value = Polygon<T>;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(POLYGON)
    }

    /// Accept Geometry enum.
    ///
    /// Recommended for self-describing formats, which most GIS formats are.
    fn visit_enum<A: EnumAccess<'de>>(self, geometry: A) -> Result<Self::Value, A::Error> {
        match geometry.variant()? {
            // Extract Polygon from Geometry enum.
            // Expects Point::deserialize called recursively and then Self::visitd_seq called.
            (POLYGON, polygon) => polygon.newtype_variant(),
            (name, _) => Err(Error::unknown_variant(name, &[POLYGON])),
        }
    }

    /// Accept newtype and flatten inner sequence.
    fn visit_newtype_struct<D: Deserializer<'de>>(self, de: D) -> Result<Self::Value, D::Error> {
        de.deserialize_seq(self)
    }

    /// Accept sequence of LineStrings.
    fn visit_seq<A: SeqAccess<'de>>(self, seq: A) -> Result<Self::Value, A::Error> {
        T::from_linestring_seq(seq).map(Polygon)
    }
}

pub trait FromLineStringSeq: Sized {
    fn from_linestring_seq<'de, A: SeqAccess<'de>>(seq: A) -> Result<Self, A::Error>;
}
