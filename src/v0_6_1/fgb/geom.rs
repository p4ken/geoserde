use serde::de::{value::SeqDeserializer, Error, IntoDeserializer, Visitor};

use crate::v0_6_1::fgb::iter::PointIter;

pub struct GeometryDeserializer<'de> {
    geom: flatgeobuf::Geometry<'de>,
}

// LineStringのデシリアライザ
struct LineStringDeserializer<'de> {
    geom: flatgeobuf::Geometry<'de>,
    start: usize,
    end: usize,
}

impl<'de> serde::Deserializer<'de> for LineStringDeserializer<'de> {
    type Error = serde::de::value::Error;

    fn deserialize_any<V: Visitor<'de>>(self, _: V) -> Result<V::Value, Self::Error> {
        Err(Error::custom("expected LineString"))
    }

    fn deserialize_newtype_struct<V: Visitor<'de>>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        if name == "geoserde::LineString" {
            // start..end の範囲のポイントのみを取得
            let points = PointIter::new(self.geom)
                .skip(self.start)
                .take(self.end - self.start);
            visitor.visit_newtype_struct(SeqDeserializer::new(points))
        } else {
            Err(Error::invalid_type(
                serde::de::Unexpected::Other(name),
                &"geoserde::LineString",
            ))
        }
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string
        bytes byte_buf option unit unit_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

impl<'de> IntoDeserializer<'de> for LineStringDeserializer<'de> {
    type Deserializer = Self;

    fn into_deserializer(self) -> Self {
        self
    }
}

impl<'de> GeometryDeserializer<'de> {
    pub fn new(geom: flatgeobuf::Geometry<'de>) -> Self {
        Self { geom }
    }

    fn deserilize_point<V>(&self, visitor: V) -> Result<V::Value, serde::de::value::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match PointIter::new(self.geom).next() {
            Some(xy) => visitor.visit_map(xy.into_deserializer()),
            None => Err(Error::custom("fbs has no coords")),
        }
    }
}

impl<'de> serde::Deserializer<'de> for GeometryDeserializer<'de> {
    type Error = serde::de::value::Error;

    fn deserialize_any<V: Visitor<'de>>(self, _: V) -> Result<V::Value, Self::Error> {
        Err(Error::custom("expected geometry type"))
    }

    fn deserialize_newtype_struct<V: Visitor<'de>>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        match name {
            "geoserde::LineString" => {
                visitor.visit_newtype_struct(PointIter::new(self.geom).into_deserializer())
            }

            // Polygonは LineString のシーケンスとして表現される
            // 各リングが geoserde::LineString としてデシリアライズされる
            // 全てのリング(exterior + interiors)に対応
            "geoserde::Polygon" => {
                if let Some(ends) = self.geom.ends() {
                    let mut rings = Vec::new();
                    let mut start = 0;

                    for end in ends {
                        rings.push(LineStringDeserializer {
                            geom: self.geom,
                            start,
                            end: end as usize,
                        });
                        start = end as usize;
                    }

                    visitor.visit_newtype_struct(SeqDeserializer::new(rings.into_iter()))
                } else {
                    Err(Error::custom("Polygon has no ends"))
                }
            }
            _ => self.deserialize_any(visitor),
        }
    }

    fn deserialize_struct<V: Visitor<'de>>(
        self,
        name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        match name {
            "geoserde::Point" => self.deserilize_point(visitor),
            _ => self.deserialize_any(visitor),
        }
    }

    fn deserialize_ignored_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_unit()
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string
        bytes byte_buf option unit unit_struct seq tuple
        tuple_struct map enum identifier
    }
}
