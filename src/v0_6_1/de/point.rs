use serde::de::{IntoDeserializer, MapAccess, Visitor};

use crate::v0_6_1::{Point, POINT};

pub fn deserialize_point<'de, D>(de: D) -> Result<Point, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    enum __Field {
        __field0,
        __field1,
        __field2,
        __field3,
        __ignore,
    }
    #[doc(hidden)]
    struct __FieldVisitor;
    #[automatically_derived]
    impl<'de> serde::de::Visitor<'de> for __FieldVisitor {
        type Value = __Field;
        fn expecting(
            &self,
            __formatter: &mut serde::__private228::Formatter,
        ) -> serde::__private228::fmt::Result {
            serde::__private228::Formatter::write_str(__formatter, "field identifier")
        }
        fn visit_str<__E>(self, __value: &str) -> serde::__private228::Result<Self::Value, __E>
        where
            __E: serde::de::Error,
        {
            match __value {
                "x" => serde::__private228::Ok(__Field::__field0),
                "y" => serde::__private228::Ok(__Field::__field1),
                "z" => serde::__private228::Ok(__Field::__field2),
                "m" => serde::__private228::Ok(__Field::__field3),
                _ => serde::__private228::Ok(__Field::__ignore),
            }
        }
    }
    #[automatically_derived]
    impl<'de> serde::Deserialize<'de> for __Field {
        #[inline]
        fn deserialize<__D>(__deserializer: __D) -> serde::__private228::Result<Self, __D::Error>
        where
            __D: serde::Deserializer<'de>,
        {
            serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
        }
    }
    #[doc(hidden)]
    struct __Visitor<'de> {
        marker: serde::__private228::PhantomData<Point>,
        lifetime: serde::__private228::PhantomData<&'de ()>,
    }
    #[automatically_derived]
    impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
        type Value = Point;
        fn expecting(
            &self,
            __formatter: &mut serde::__private228::Formatter,
        ) -> serde::__private228::fmt::Result {
            serde::__private228::Formatter::write_str(__formatter, POINT)
        }
        #[inline]
        fn visit_seq<__A>(
            self,
            mut __seq: __A,
        ) -> serde::__private228::Result<Self::Value, __A::Error>
        where
            __A: serde::de::SeqAccess<'de>,
        {
            // Flatten single Point sequence recursively
            let point = match serde::de::SeqAccess::next_element::<Point>(&mut __seq)? {
                serde::__private228::Some(__value) => __value,
                serde::__private228::None => {
                    return serde::__private228::Err(serde::de::Error::invalid_length(
                        0usize,
                        &"struct Point with 1 element",
                    ));
                }
            };
            serde::__private228::Ok(point)
        }
        #[inline]
        fn visit_map<__A>(
            self,
            mut __map: __A,
        ) -> serde::__private228::Result<Self::Value, __A::Error>
        where
            __A: serde::de::MapAccess<'de>,
        {
            let mut __field0: serde::__private228::Option<f64> = serde::__private228::None;
            let mut __field1: serde::__private228::Option<f64> = serde::__private228::None;
            let mut __field2: serde::__private228::Option<Option<f64>> = serde::__private228::None;
            let mut __field3: serde::__private228::Option<Option<f64>> = serde::__private228::None;
            while let serde::__private228::Some(__key) =
                serde::de::MapAccess::next_key::<__Field>(&mut __map)?
            {
                match __key {
                    __Field::__field0 => {
                        if serde::__private228::Option::is_some(&__field0) {
                            return serde::__private228::Err(
                                <__A::Error as serde::de::Error>::duplicate_field("x"),
                            );
                        }
                        __field0 = serde::__private228::Some(serde::de::MapAccess::next_value::<
                            f64,
                        >(&mut __map)?);
                    }
                    __Field::__field1 => {
                        if serde::__private228::Option::is_some(&__field1) {
                            return serde::__private228::Err(
                                <__A::Error as serde::de::Error>::duplicate_field("y"),
                            );
                        }
                        __field1 = serde::__private228::Some(serde::de::MapAccess::next_value::<
                            f64,
                        >(&mut __map)?);
                    }
                    __Field::__field2 => {
                        if serde::__private228::Option::is_some(&__field2) {
                            return serde::__private228::Err(
                                <__A::Error as serde::de::Error>::duplicate_field("z"),
                            );
                        }
                        __field2 = serde::__private228::Some(serde::de::MapAccess::next_value::<
                            Option<f64>,
                        >(&mut __map)?);
                    }
                    __Field::__field3 => {
                        if serde::__private228::Option::is_some(&__field3) {
                            return serde::__private228::Err(
                                <__A::Error as serde::de::Error>::duplicate_field("m"),
                            );
                        }
                        __field3 = serde::__private228::Some(serde::de::MapAccess::next_value::<
                            Option<f64>,
                        >(&mut __map)?);
                    }
                    _ => {
                        let _ =
                            serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(&mut __map)?;
                    }
                }
            }
            let __field0 = match __field0 {
                serde::__private228::Some(__field0) => __field0,
                serde::__private228::None => serde::__private228::de::missing_field("x")?,
            };
            let __field1 = match __field1 {
                serde::__private228::Some(__field1) => __field1,
                serde::__private228::None => serde::__private228::de::missing_field("y")?,
            };
            let __field2 = match __field2 {
                serde::__private228::Some(__field2) => __field2,
                serde::__private228::None => serde::__private228::de::missing_field("z")?,
            };
            let __field3 = match __field3 {
                serde::__private228::Some(__field3) => __field3,
                serde::__private228::None => serde::__private228::de::missing_field("m")?,
            };
            serde::__private228::Ok(Point {
                x: __field0,
                y: __field1,
                z: __field2,
                m: __field3,
            })
        }

        // Extract Point variant from the enum.
        fn visit_enum<A>(self, geometry: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::EnumAccess<'de>,
        {
            match geometry.variant().unwrap() {
                (POINT, point) => serde::de::VariantAccess::newtype_variant(point),
                (id, _) => Err(serde::de::Error::invalid_value(
                    serde::de::Unexpected::Other(id),
                    &self,
                )),
            }
        }
    }
    #[doc(hidden)]
    const FIELDS: &'static [&'static str] = &["x", "y", "z", "m"];
    serde::Deserializer::deserialize_struct(
        de,
        POINT,
        FIELDS,
        __Visitor {
            marker: serde::__private228::PhantomData::<Point>,
            lifetime: serde::__private228::PhantomData,
        },
    )
}

pub struct PointDeserializer {
    point: crate::v0_6_1::Point,
    key: Option<Key>,
}

impl PointDeserializer {
    pub fn new(point: crate::v0_6_1::Point) -> Self {
        Self { point, key: None }
    }
}

impl<'de> serde::Deserializer<'de> for PointDeserializer {
    type Error = serde::de::value::Error;

    fn deserialize_any<V: Visitor<'de>>(self, _: V) -> Result<V::Value, Self::Error> {
        Err(serde::de::Error::custom("expected geoserde::Point"))
    }

    fn deserialize_bool<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_any(visitor)
    }

    fn deserialize_i8<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_any(visitor)
    }

    fn deserialize_i16<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_any(visitor)
    }

    fn deserialize_i32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_any(visitor)
    }

    fn deserialize_i64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_any(visitor)
    }

    fn deserialize_u8<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_any(visitor)
    }

    fn deserialize_u16<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_any(visitor)
    }

    fn deserialize_u32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_any(visitor)
    }

    fn deserialize_u64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_any(visitor)
    }

    fn deserialize_f32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_any(visitor)
    }

    fn deserialize_f64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_any(visitor)
    }

    fn deserialize_char<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_any(visitor)
    }

    fn deserialize_str<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_any(visitor)
    }

    fn deserialize_string<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_any(visitor)
    }

    fn deserialize_bytes<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_any(visitor)
    }

    fn deserialize_byte_buf<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_any(visitor)
    }

    fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_any(visitor)
    }

    fn deserialize_unit<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_any(visitor)
    }

    fn deserialize_unit_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.deserialize_any(visitor)
    }

    fn deserialize_newtype_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.deserialize_any(visitor)
    }

    fn deserialize_seq<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_any(visitor)
    }

    fn deserialize_tuple<V: Visitor<'de>>(
        self,
        _: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.deserialize_any(visitor)
    }

    fn deserialize_tuple_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.deserialize_any(visitor)
    }

    fn deserialize_map<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_any(visitor)
    }

    fn deserialize_struct<V: Visitor<'de>>(
        self,
        name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        match name {
            "geoserde::Point" => visitor.visit_map(self),
            _ => self.deserialize_any(visitor),
        }
    }

    fn deserialize_enum<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.deserialize_any(visitor)
    }

    fn deserialize_identifier<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_any(visitor)
    }

    fn deserialize_ignored_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_any(visitor)
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
            .map(|key| seed.deserialize(key.as_str().into_deserializer()))
            .transpose()
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        match self.key.unwrap() {
            Key::X => seed.deserialize(self.point.x.into_deserializer()),
            Key::Y => seed.deserialize(self.point.y.into_deserializer()),
            Key::Z => seed.deserialize(self.point.z.unwrap().into_deserializer()),
            Key::M => seed.deserialize(self.point.m.unwrap().into_deserializer()),
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
