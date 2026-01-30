use serde::{Deserialize, Serialize};

mod de;
#[cfg(feature = "flatgeobuf")]
pub mod fgb;
mod geo;

pub fn serialize<S: serde::Serializer>(
    geom: impl SerializeGeometry,
    ser: S,
) -> Result<S::Ok, S::Error> {
    geom.serialize(ser)
}

pub fn deserialize<'a, D: serde::Deserializer<'a>, G: DeserializeGeometry>(
    de: D,
) -> Result<G, D::Error> {
    G::deserialize_geometry(de)
}

pub trait SerializeGeometry: Serialize {}
impl SerializeGeometry for geo_types::Point {}
impl<T: SerializeGeometry> SerializeGeometry for &T {}

pub trait DeserializeGeometry: Sized {
    fn deserialize_geometry<'a, D: serde::Deserializer<'a>>(de: D) -> Result<Self, D::Error>;
}

/// Feature to deserialize a geometry with no properties
#[derive(Debug, Deserialize)]
pub struct GeometrySink<G: DeserializeGeometry> {
    #[serde(deserialize_with = "deserialize")]
    #[serde(rename = "geoserde::geometry")]
    /// Deserialized geometry
    pub g: G,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "geoserde::Geometry")]
pub enum Geometry {
    #[serde(rename="geoserde::Point")]
    Point(Point),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: Option<f64>,
    pub m: Option<f64>,
}

#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    impl<'de> _serde::Deserialize<'de> for Point {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private228::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
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
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private228::Formatter,
                ) -> _serde::__private228::fmt::Result {
                    _serde::__private228::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private228::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private228::Ok(__Field::__field0),
                        1u64 => _serde::__private228::Ok(__Field::__field1),
                        2u64 => _serde::__private228::Ok(__Field::__field2),
                        3u64 => _serde::__private228::Ok(__Field::__field3),
                        _ => _serde::__private228::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private228::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "x" => _serde::__private228::Ok(__Field::__field0),
                        "y" => _serde::__private228::Ok(__Field::__field1),
                        "z" => _serde::__private228::Ok(__Field::__field2),
                        "m" => _serde::__private228::Ok(__Field::__field3),
                        _ => _serde::__private228::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private228::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"x" => _serde::__private228::Ok(__Field::__field0),
                        b"y" => _serde::__private228::Ok(__Field::__field1),
                        b"z" => _serde::__private228::Ok(__Field::__field2),
                        b"m" => _serde::__private228::Ok(__Field::__field3),
                        _ => _serde::__private228::Ok(__Field::__ignore),
                    }
                }
            }
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private228::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private228::PhantomData<Point>,
                lifetime: _serde::__private228::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = Point;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private228::Formatter,
                ) -> _serde::__private228::fmt::Result {
                    _serde::__private228::Formatter::write_str(
                        __formatter,
                        "struct Point",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private228::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        f64,
                    >(&mut __seq)? {
                        _serde::__private228::Some(__value) => __value,
                        _serde::__private228::None => {
                            return _serde::__private228::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct Point with 4 elements",
                                ),
                            );
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<
                        f64,
                    >(&mut __seq)? {
                        _serde::__private228::Some(__value) => __value,
                        _serde::__private228::None => {
                            return _serde::__private228::Err(
                                _serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct Point with 4 elements",
                                ),
                            );
                        }
                    };
                    let __field2 = match _serde::de::SeqAccess::next_element::<
                        Option<f64>,
                    >(&mut __seq)? {
                        _serde::__private228::Some(__value) => __value,
                        _serde::__private228::None => {
                            return _serde::__private228::Err(
                                _serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct Point with 4 elements",
                                ),
                            );
                        }
                    };
                    let __field3 = match _serde::de::SeqAccess::next_element::<
                        Option<f64>,
                    >(&mut __seq)? {
                        _serde::__private228::Some(__value) => __value,
                        _serde::__private228::None => {
                            return _serde::__private228::Err(
                                _serde::de::Error::invalid_length(
                                    3usize,
                                    &"struct Point with 4 elements",
                                ),
                            );
                        }
                    };
                    _serde::__private228::Ok(Point {
                        x: __field0,
                        y: __field1,
                        z: __field2,
                        m: __field3,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private228::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private228::Option<f64> = _serde::__private228::None;
                    let mut __field1: _serde::__private228::Option<f64> = _serde::__private228::None;
                    let mut __field2: _serde::__private228::Option<Option<f64>> = _serde::__private228::None;
                    let mut __field3: _serde::__private228::Option<Option<f64>> = _serde::__private228::None;
                    while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private228::Option::is_some(&__field0) {
                                    return _serde::__private228::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("x"),
                                    );
                                }
                                __field0 = _serde::__private228::Some(
                                    _serde::de::MapAccess::next_value::<f64>(&mut __map)?,
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private228::Option::is_some(&__field1) {
                                    return _serde::__private228::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("y"),
                                    );
                                }
                                __field1 = _serde::__private228::Some(
                                    _serde::de::MapAccess::next_value::<f64>(&mut __map)?,
                                );
                            }
                            __Field::__field2 => {
                                if _serde::__private228::Option::is_some(&__field2) {
                                    return _serde::__private228::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("z"),
                                    );
                                }
                                __field2 = _serde::__private228::Some(
                                    _serde::de::MapAccess::next_value::<
                                        Option<f64>,
                                    >(&mut __map)?,
                                );
                            }
                            __Field::__field3 => {
                                if _serde::__private228::Option::is_some(&__field3) {
                                    return _serde::__private228::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("m"),
                                    );
                                }
                                __field3 = _serde::__private228::Some(
                                    _serde::de::MapAccess::next_value::<
                                        Option<f64>,
                                    >(&mut __map)?,
                                );
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private228::Some(__field0) => __field0,
                        _serde::__private228::None => {
                            _serde::__private228::de::missing_field("x")?
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private228::Some(__field1) => __field1,
                        _serde::__private228::None => {
                            _serde::__private228::de::missing_field("y")?
                        }
                    };
                    let __field2 = match __field2 {
                        _serde::__private228::Some(__field2) => __field2,
                        _serde::__private228::None => {
                            _serde::__private228::de::missing_field("z")?
                        }
                    };
                    let __field3 = match __field3 {
                        _serde::__private228::Some(__field3) => __field3,
                        _serde::__private228::None => {
                            _serde::__private228::de::missing_field("m")?
                        }
                    };
                    _serde::__private228::Ok(Point {
                        x: __field0,
                        y: __field1,
                        z: __field2,
                        m: __field3,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["x", "y", "z", "m"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "geoserde::Point",
                FIELDS,
                __Visitor {
                    marker: _serde::__private228::PhantomData::<Point>,
                    lifetime: _serde::__private228::PhantomData,
                },
            )
        }
    }
};

impl<'de> serde::de::IntoDeserializer<'de> for Point {
    type Deserializer = de::PointDeserializer;

    fn into_deserializer(self) -> Self::Deserializer {
        de::PointDeserializer::new(self)
    }
}

#[derive(Debug, Clone, Default)]
pub struct LineString<T>(pub T);
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

#[derive(Debug, Clone, Copy, Default)]
pub struct Polygon<T>(pub T);
impl<'de, T: FromLineStringSeq> Deserialize<'de>  for Polygon<T> {
    fn deserialize<D: serde::Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        struct LineStringSeqVisitor<T>(std::marker::PhantomData<T>);
        impl<'de , T: FromLineStringSeq> serde::de::Visitor<'de> for LineStringSeqVisitor<T> {
            type Value = T;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str(&"a sequence of geoserde::LineString")
            }
            fn visit_seq<A: serde::de::SeqAccess<'de>>(self, seq: A) -> Result<Self::Value, A::Error> {
                T::from_linestring_seq(seq)
            }
        }

        struct NewtypeVisitor<T>(std::marker::PhantomData<T>);
        impl<'de, T: FromLineStringSeq> serde::de::Visitor<'de> for NewtypeVisitor<T> {
            type Value = T;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("a newtype struct")
            }
            fn visit_newtype_struct<D: serde::Deserializer<'de>>(self, de: D) -> Result<Self::Value, D::Error> {
                de.deserialize_seq(LineStringSeqVisitor(std::marker::PhantomData))
            }
        }
        de.deserialize_newtype_struct("geoserde::Polygon", NewtypeVisitor(std::marker::PhantomData)).map(Self)
    }
}

pub trait FromLineStringSeq : Sized {
    fn from_linestring_seq<'de, A: serde::de::SeqAccess<'de> >(seq: A) -> Result<Self, A::Error>;
}
