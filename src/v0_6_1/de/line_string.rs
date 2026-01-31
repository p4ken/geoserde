use serde::Deserialize;

use crate::v0_6_1::{LineString, Point};

impl<'de, T: FromPointSeq> Deserialize<'de>  for LineString<T> {
    fn deserialize<D: serde::Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        struct PointSeqVisitor<T>(std::marker::PhantomData<T>);
        impl<'de , T: FromPointSeq> serde::de::Visitor<'de> for PointSeqVisitor<T> {
            type Value = T;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str(&"a sequence of geoserde::Point")
            }

            fn visit_seq<A: serde::de::SeqAccess<'de>>(self, seq: A) -> Result<Self::Value, A::Error> {
                T::from_point_seq(seq)
            }
        }

        struct NewtypeVisitor<T>(std::marker::PhantomData<T>);
        impl<'de, T: FromPointSeq> serde::de::Visitor<'de> for NewtypeVisitor<T> {
            type Value = T;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("a newtype struct")
            }

            fn visit_newtype_struct<D: serde::Deserializer<'de>>(self, de: D) -> Result<Self::Value, D::Error> {
                de.deserialize_seq(PointSeqVisitor(std::marker::PhantomData))
            }
        }
        de.deserialize_newtype_struct("geoserde::LineString", NewtypeVisitor(std::marker::PhantomData)).map(Self)
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
