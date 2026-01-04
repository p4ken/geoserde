use serde::{Deserialize, Serialize};

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
    G::deserialize(de)
}

/// Feature to deserialize a geometry with no properties
#[derive(Debug, Deserialize)]
pub struct GeometrySink<G: DeserializeGeometry> {
    #[serde(deserialize_with = "deserialize")]
    #[serde(rename = "geoserde::geometry")]
    /// Deserialized geometry
    pub g: G,
}

pub trait SerializeGeometry: Serialize {}
impl SerializeGeometry for geo_types::Point {}
impl<T: SerializeGeometry> SerializeGeometry for &T {}

pub trait DeserializeGeometry: Sized {
    fn deserialize<'a, D: serde::Deserializer<'a>>(de: D) -> Result<Self, D::Error>;
}

#[derive(Debug, Clone, Copy, Default, Deserialize)]
#[serde(rename = "geoserde::Point")]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: Option<f64>,
    pub m: Option<f64>,
}

pub struct LineString<T>(pub T);
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de, T> _serde::Deserialize<'de> for LineString<T>
    where
        T: _serde::Deserialize<'de>,
    {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[doc(hidden)]
            struct __Visitor<'de, T>
            where
                T: _serde::Deserialize<'de>,
            {
                marker: _serde::__private228::PhantomData<LineString<T>>,
                lifetime: _serde::__private228::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de, T> _serde::de::Visitor<'de> for __Visitor<'de, T>
            where
                T: _serde::Deserialize<'de>,
            {
                type Value = LineString<T>;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private228::Formatter,
                ) -> _serde::__private228::fmt::Result {
                    _serde::__private228::Formatter::write_str(
                        __formatter,
                        "tuple struct LineString",
                    )
                }
                #[inline]
                fn visit_newtype_struct<__E>(
                    self,
                    __e: __E,
                ) -> _serde::__private228::Result<Self::Value, __E::Error>
                where
                    __E: _serde::Deserializer<'de>,
                {
                    let __field0: T = <T as _serde::Deserialize>::deserialize(__e)?;
                    _serde::__private228::Ok(LineString(__field0))
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private228::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<T>(&mut __seq)? {
                        _serde::__private228::Some(__value) => __value,
                        _serde::__private228::None => {
                            return _serde::__private228::Err(_serde::de::Error::invalid_length(
                                0usize,
                                &"tuple struct LineString with 1 element",
                            ));
                        }
                    };
                    _serde::__private228::Ok(LineString(__field0))
                }
            }
            _serde::Deserializer::deserialize_newtype_struct(
                __deserializer,
                "geoserde::LineString",
                __Visitor {
                    marker: _serde::__private228::PhantomData::<LineString<T>>,
                    lifetime: _serde::__private228::PhantomData,
                },
            )
        }
    }
};
