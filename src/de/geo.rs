#![cfg(feature = "geo-types")]

use geo_traits::{
    GeometryTrait, GeometryType, MultiPointTrait, PointTrait,
    to_geo::{ToGeoLineString, ToGeoMultiLineString, ToGeoPoint, ToGeoPolygon},
};

use crate::de::{DeserializeGeometry, GeometryError, GeometryOptions};

// Calling per-variant traits such as `ToGeoPoint` on the variant taken out by `as_type()`
// does not go through `GeometryTrait`, so it avoids the trait solver overflow that
// `ToGeoGeometry` hits (rustc #128887).
impl DeserializeGeometry for geo_types::Point {
    fn deserialize_geometry<T: GeometryTrait<T = f64>>(
        source: T,
        _options: &GeometryOptions,
    ) -> Result<Self, GeometryError> {
        match source.as_type() {
            GeometryType::Point(p) => point(p),
            _ => Err(GeometryError::type_mismatch("Point", &source)),
        }
    }
}

impl DeserializeGeometry for geo_types::MultiPoint {
    fn deserialize_geometry<T: GeometryTrait<T = f64>>(
        source: T,
        options: &GeometryOptions,
    ) -> Result<Self, GeometryError> {
        match source.as_type() {
            GeometryType::Point(p) if options.promote_to_multi() => {
                Ok(geo_types::MultiPoint::new(vec![point(p)?]))
            }
            GeometryType::MultiPoint(mp) => mp.points().map(|p| point(&p)).collect(),
            _ => Err(GeometryError::type_mismatch("MultiPoint", &source)),
        }
    }
}

impl DeserializeGeometry for geo_types::LineString {
    fn deserialize_geometry<T: GeometryTrait<T = f64>>(
        source: T,
        _options: &GeometryOptions,
    ) -> Result<Self, GeometryError> {
        match source.as_type() {
            GeometryType::LineString(ls) => Ok(ls.to_line_string()),
            _ => Err(GeometryError::type_mismatch("LineString", &source)),
        }
    }
}

impl DeserializeGeometry for geo_types::MultiLineString {
    fn deserialize_geometry<T: GeometryTrait<T = f64>>(
        source: T,
        options: &GeometryOptions,
    ) -> Result<Self, GeometryError> {
        match source.as_type() {
            GeometryType::LineString(ls) if options.promote_to_multi() => {
                Ok(geo_types::MultiLineString::new(vec![ls.to_line_string()]))
            }
            GeometryType::MultiLineString(mls) => Ok(mls.to_multi_line_string()),
            _ => Err(GeometryError::type_mismatch("MultiLineString", &source)),
        }
    }
}

impl DeserializeGeometry for geo_types::Polygon {
    fn deserialize_geometry<T: GeometryTrait<T = f64>>(
        source: T,
        _options: &GeometryOptions,
    ) -> Result<Self, GeometryError> {
        match source.as_type() {
            GeometryType::Polygon(p) => Ok(p.to_polygon()),
            _ => Err(GeometryError::type_mismatch("Polygon", &source)),
        }
    }
}

/// Fails on an empty point instead of panicking like [`ToGeoPoint::to_point`].
fn point<P: PointTrait<T = f64>>(p: &P) -> Result<geo_types::Point, GeometryError> {
    p.try_to_point()
        .ok_or_else(|| GeometryError::custom("geo-types cannot represent an empty point"))
}
