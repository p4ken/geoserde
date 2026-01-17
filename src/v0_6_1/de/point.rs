use serde::de::{
    value::{F64Deserializer, StrDeserializer},
    MapAccess, Visitor,
};

pub struct PointDeserializer {
    point: crate::v0_6_1::Point,
    key: Option<Key>,
}

impl PointDeserializer {
    pub fn new(x: f64, y: f64, z: Option<f64>, m: Option<f64>) -> Self {
        Self {
            point: crate::v0_6_1::Point { x, y, z, m },
            key: None,
        }
    }
}

impl<'de> serde::Deserializer<'de> for &mut PointDeserializer {
    type Error = serde::de::value::Error;

    fn deserialize_any<V: Visitor<'de>>(self, _: V) -> Result<V::Value, Self::Error> {
        todo!()
    }

    fn deserialize_bool<V: Visitor<'de>>(self, _: V) -> Result<V::Value, Self::Error> {
        todo!()
    }

    fn deserialize_i8<V: Visitor<'de>>(self, _: V) -> Result<V::Value, Self::Error> {
        todo!()
    }

    fn deserialize_i16<V: Visitor<'de>>(self, _: V) -> Result<V::Value, Self::Error> {
        todo!()
    }

    fn deserialize_i32<V: Visitor<'de>>(self, _: V) -> Result<V::Value, Self::Error> {
        todo!()
    }

    fn deserialize_i64<V: Visitor<'de>>(self, _: V) -> Result<V::Value, Self::Error> {
        todo!()
    }

    fn deserialize_u8<V: Visitor<'de>>(self, _: V) -> Result<V::Value, Self::Error> {
        todo!()
    }

    fn deserialize_u16<V: Visitor<'de>>(self, _: V) -> Result<V::Value, Self::Error> {
        todo!()
    }

    fn deserialize_u32<V: Visitor<'de>>(self, _: V) -> Result<V::Value, Self::Error> {
        todo!()
    }

    fn deserialize_u64<V: Visitor<'de>>(self, _: V) -> Result<V::Value, Self::Error> {
        todo!()
    }

    fn deserialize_f32<V: Visitor<'de>>(self, _: V) -> Result<V::Value, Self::Error> {
        todo!()
    }

    fn deserialize_f64<V: Visitor<'de>>(self, _: V) -> Result<V::Value, Self::Error> {
        todo!()
    }

    fn deserialize_char<V: Visitor<'de>>(self, _: V) -> Result<V::Value, Self::Error> {
        todo!()
    }

    fn deserialize_str<V: Visitor<'de>>(self, _: V) -> Result<V::Value, Self::Error> {
        todo!()
    }

    fn deserialize_string<V: Visitor<'de>>(self, _: V) -> Result<V::Value, Self::Error> {
        todo!()
    }

    fn deserialize_bytes<V: Visitor<'de>>(self, _: V) -> Result<V::Value, Self::Error> {
        todo!()
    }

    fn deserialize_byte_buf<V: Visitor<'de>>(self, _: V) -> Result<V::Value, Self::Error> {
        todo!()
    }

    fn deserialize_option<V: Visitor<'de>>(self, _: V) -> Result<V::Value, Self::Error> {
        todo!()
    }

    fn deserialize_unit<V: Visitor<'de>>(self, _: V) -> Result<V::Value, Self::Error> {
        todo!()
    }

    fn deserialize_unit_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _: V,
    ) -> Result<V::Value, Self::Error> {
        todo!()
    }

    fn deserialize_newtype_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _: V,
    ) -> Result<V::Value, Self::Error> {
        todo!()
    }

    fn deserialize_seq<V: Visitor<'de>>(self, _: V) -> Result<V::Value, Self::Error> {
        todo!()
    }

    fn deserialize_tuple<V: Visitor<'de>>(self, _: usize, _: V) -> Result<V::Value, Self::Error> {
        todo!()
    }

    fn deserialize_tuple_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _len: usize,
        _: V,
    ) -> Result<V::Value, Self::Error> {
        todo!()
    }

    fn deserialize_map<V: Visitor<'de>>(self, _: V) -> Result<V::Value, Self::Error> {
        todo!()
    }

    fn deserialize_struct<V: Visitor<'de>>(
        self,
        name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        match name {
            "geoserde::Point" => visitor.visit_map(self),
            _ => todo!(),
        }
    }

    fn deserialize_enum<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _: &'static [&'static str],
        _: V,
    ) -> Result<V::Value, Self::Error> {
        todo!()
    }

    fn deserialize_identifier<V: Visitor<'de>>(self, _: V) -> Result<V::Value, Self::Error> {
        todo!()
    }

    fn deserialize_ignored_any<V: Visitor<'de>>(self, _: V) -> Result<V::Value, Self::Error> {
        todo!()
    }
}

impl<'de> MapAccess<'de> for PointDeserializer {
    type Error = serde::de::value::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        self.key = match self.key {
            None => Some(Key::X),
            Some(Key::X) => Some(Key::Y),
            Some(Key::Y) => Some(Key::Z),
            Some(Key::Z) => Some(Key::M),
            Some(Key::M) => None,
        };
        if self.key == Some(Key::Z) && self.point.z.is_none() {
            self.key = Some(Key::M);
        }
        if self.key == Some(Key::M) && self.point.m.is_none() {
            self.key = None;
        }

        self.key
            .map(|key| seed.deserialize(StrDeserializer::new(key.as_str())))
            .transpose()
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        match self.key.unwrap() {
            Key::X => seed.deserialize(F64Deserializer::new(self.point.x)),
            Key::Y => seed.deserialize(F64Deserializer::new(self.point.y)),
            Key::Z => seed.deserialize(F64Deserializer::new(self.point.z.unwrap())),
            Key::M => seed.deserialize(F64Deserializer::new(self.point.m.unwrap())),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Key {
    X,
    Y,
    Z,
    M,
}
impl Key {
    fn as_str(&self) -> &'static str {
        match self {
            Key::X => "x",
            Key::Y => "y",
            Key::Z => "z",
            Key::M => "m",
        }
    }
}
