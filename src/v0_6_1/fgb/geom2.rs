use std::fmt::Display;

use serde::de::{DeserializeSeed, Error, IntoDeserializer, StdError, Unexpected, Visitor};

use crate::v0_6_1::fgb::coord2::{LineStringIter, PointIter, PolygonIter};

pub struct GeometryDeserializer<'a> {
    geom: flatgeobuf::Geometry<'a>,
    geom_type: flatgeobuf::GeometryType,
}

impl<'de> GeometryDeserializer<'de> {
    const VARIANT: Unexpected<'static> = Unexpected::NewtypeVariant;

    pub fn new(geom: flatgeobuf::Geometry<'de>, header: flatgeobuf::GeometryType) -> Self {
        let mut geom_type = geom.type_();
        if geom_type == flatgeobuf::GeometryType::Unknown {
            geom_type = header
        };
        Self { geom, geom_type }
    }
}

impl<'de> serde::de::EnumAccess<'de> for GeometryDeserializer<'de> {
    type Error = GeometryError;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        use flatgeobuf::GeometryType;
        let variant = match self.geom_type {
            GeometryType::Point => "geoserde::Point",
            GeometryType::LineString => "geoserde::LineString",
            GeometryType::Polygon => "geoserde::Polygon",
            GeometryType::MultiPoint => "geoserde::MultiPoint",
            GeometryType::MultiLineString => "geoserde::MultiLineString",
            GeometryType::MultiPolygon => "geoserde::MultiPolygon",
            GeometryType::Unknown
            | GeometryType::GeometryCollection
            | GeometryType::CircularString
            | GeometryType::CompoundCurve
            | GeometryType::CurvePolygon
            | GeometryType::MultiCurve
            | GeometryType::MultiSurface
            | GeometryType::Curve
            | GeometryType::Surface
            | GeometryType::PolyhedralSurface
            | GeometryType::TIN
            | GeometryType::Triangle
            | _ => return Err(GeometryError::Type(self.geom_type)),
        };
        let value = seed.deserialize(variant.into_deserializer())?;
        Ok((value, self))
    }
}

impl<'de> serde::de::VariantAccess<'de> for GeometryDeserializer<'de> {
    type Error = GeometryError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Err(Error::invalid_type(Self::VARIANT, &"unit variant"))
    }

    fn newtype_variant_seed<T: DeserializeSeed<'de>>(
        self,
        seed: T,
    ) -> Result<T::Value, Self::Error> {
        match self.geom.parts() {
            Some(parts) => seed.deserialize(PolygonIter::new(parts).into_deserializer()),
            None => match (
                self.geom.ends(),
                self.geom.xy(),
                self.geom.z(),
                self.geom.m(),
            ) {
                (Some(ends), xy, z, m) => {
                    seed.deserialize(LineStringIter::new(ends, xy, z, m).into_deserializer())
                }
                (None, xy, z, m) => seed.deserialize(PointIter::new(xy, z, m).into_deserializer()),
            },
        }
        .map_err(GeometryError::Coords)
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::invalid_type(Self::VARIANT, &"tuple variant"))
    }

    fn struct_variant<V: Visitor<'de>>(
        self,
        _: &'_ [&'_ str],
        _: V,
    ) -> Result<V::Value, Self::Error> {
        Err(Error::invalid_type(Self::VARIANT, &"struct variant"))
    }
}

#[derive(Debug)]
pub enum GeometryError {
    Type(flatgeobuf::GeometryType),
    Deserialize(String),
    Coords(serde::de::value::Error),
}

impl Error for GeometryError {
    fn custom<T: Display>(msg: T) -> Self {
        GeometryError::Deserialize(msg.to_string())
    }
}

impl StdError for GeometryError {}

impl Display for GeometryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Type(t) => write!(f, "unexpected geometry type {:?}", t),
            Self::Deserialize(s) => write!(f, "deserialize impl occured {}", s),
            Self::Coords(e) => e.fmt(f),
        }
    }
}
