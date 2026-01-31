use std::fmt::Display;

use serde::de::{
    value::{BorrowedStrDeserializer, EnumAccessDeserializer},
    DeserializeSeed, EnumAccess, Error, IntoDeserializer, StdError, Unexpected, VariantAccess,
    Visitor,
};

use crate::v0_6_1::fgb::coord2::{LineStringIter, PointIter, PolygonIter};

pub struct GeometryAccess<'a> {
    geom: flatgeobuf::Geometry<'a>,
    geom_type: flatgeobuf::GeometryType,
}

impl<'de> GeometryAccess<'de> {
    const VARIANT: Unexpected<'static> = Unexpected::NewtypeVariant;

    pub fn new(geom: flatgeobuf::Geometry<'de>, header: flatgeobuf::GeometryType) -> Self {
        let mut geom_type = geom.type_();
        if geom_type == flatgeobuf::GeometryType::Unknown {
            geom_type = header
        };
        Self { geom, geom_type }
    }
}

impl<'de> IntoDeserializer<'de, GeometryError> for GeometryAccess<'de> {
    type Deserializer = EnumAccessDeserializer<Self>;

    fn into_deserializer(self) -> Self::Deserializer {
        EnumAccessDeserializer::new(self)
    }
}

impl<'de> EnumAccess<'de> for GeometryAccess<'de> {
    type Error = GeometryError;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'de>,
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
        let value = seed.deserialize(BorrowedStrDeserializer::new(variant))?;
        Ok((value, self))
    }
}

impl<'de> VariantAccess<'de> for GeometryAccess<'de> {
    type Error = GeometryError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Err(Error::invalid_type(Self::VARIANT, &"unit variant"))
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
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

    fn tuple_variant<V>(self, _: usize, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::invalid_type(Self::VARIANT, &"tuple variant"))
    }

    fn struct_variant<V>(self, _: &'_ [&'_ str], _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::invalid_type(Self::VARIANT, &"struct variant"))
    }
}

#[derive(Debug)]
pub enum GeometryError {
    Type(flatgeobuf::GeometryType),
    Deserialize(serde::de::value::Error),
    Coords(serde::de::value::Error),
}

impl Error for GeometryError {
    fn custom<T: Display>(msg: T) -> Self {
        Self::Deserialize(Error::custom(msg))
    }
}

impl StdError for GeometryError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(match self {
            Self::Deserialize(e) => e,
            Self::Coords(e) => e,
            _ => None?,
        })
    }
}

impl Display for GeometryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Type(t) => write!(f, "unexpected geometry type {:?}", t),
            Self::Deserialize(_) => write!(f, "deserialize impl failed"),
            Self::Coords(_) => write!(f, "coordinates deserializer failed"),
        }
    }
}
