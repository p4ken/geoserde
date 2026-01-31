use std::marker::PhantomData;

struct LineStringSeqVisitor<T>(std::marker::PhantomData<T>);
impl<'de, T: FromLineStringSeq> serde::de::Visitor<'de> for LineStringSeqVisitor<T> {
    type Value = T;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&"a sequence of geoserde::LineString")
    }
    fn visit_seq<A: serde::de::SeqAccess<'de>>(self, seq: A) -> Result<Self::Value, A::Error> {
        T::from_linestring_seq(seq)
    }
}

pub struct PolygonVisitor<T>(PhantomData<T>);

impl<T> PolygonVisitor<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<'de, T: FromLineStringSeq> serde::de::Visitor<'de> for PolygonVisitor<T> {
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

pub trait FromLineStringSeq: Sized {
    fn from_linestring_seq<'de, A: serde::de::SeqAccess<'de>>(seq: A) -> Result<Self, A::Error>;
}
