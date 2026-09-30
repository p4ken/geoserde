#![cfg(feature = "geo-types")]

use geo_traits::{
    CoordTrait, GeometryTrait, GeometryType, LineStringTrait, MultiLineStringTrait,
    MultiPointTrait, PointTrait, PolygonTrait,
    to_geo::{ToGeoLineString, ToGeoMultiLineString, ToGeoPoint, ToGeoPolygon},
};

use crate::de::{DeserializeGeometry, GeometryError};

// Calling per-variant traits such as `ToGeoPoint` on the variant taken out by `as_type()`
// does not go through `GeometryTrait`, so it avoids the trait solver overflow that
// `ToGeoGeometry` hits (rustc #128887).
impl DeserializeGeometry for geo_types::Point {
    fn deserialize_geometry<T: GeometryTrait<T = f64>>(source: T) -> Result<Self, GeometryError> {
        match source.as_type() {
            GeometryType::Point(p) => point(p),
            GeometryType::MultiPoint(mp) => only(mp.points())
                .ok_or(GeometryError::type_mismatch("Point", &source))
                .and_then(|p| point(&p)),
            _ => Err(GeometryError::type_mismatch("Point", &source)),
        }
    }
}

impl DeserializeGeometry for geo_types::MultiPoint {
    fn deserialize_geometry<T: GeometryTrait<T = f64>>(source: T) -> Result<Self, GeometryError> {
        match source.as_type() {
            GeometryType::Point(p) => Ok(geo_types::MultiPoint::new(vec![point(p)?])),
            GeometryType::MultiPoint(mp) => mp.points().map(|p| point(&p)).collect(),
            GeometryType::LineString(ls) => Ok(geo_types::MultiPoint::new(
                ls.coords()
                    .map(|c| geo_types::Point::new(c.x(), c.y()))
                    .collect(),
            )),
            GeometryType::MultiLineString(mls) => {
                let ls = only(mls.line_strings())
                    .ok_or(GeometryError::type_mismatch("MultiPoint", &source))?;
                Ok(geo_types::MultiPoint::new(
                    ls.coords()
                        .map(|c| geo_types::Point::new(c.x(), c.y()))
                        .collect(),
                ))
            }
            GeometryType::Polygon(poly) => {
                let ring =
                    only_ring(poly).ok_or(GeometryError::type_mismatch("MultiPoint", &source))?;
                Ok(geo_types::MultiPoint::new(
                    ring.coords()
                        .map(|c| geo_types::Point::new(c.x(), c.y()))
                        .collect(),
                ))
            }
            _ => Err(GeometryError::type_mismatch("MultiPoint", &source)),
        }
    }
}

impl DeserializeGeometry for geo_types::LineString {
    fn deserialize_geometry<T: GeometryTrait<T = f64>>(source: T) -> Result<Self, GeometryError> {
        match source.as_type() {
            GeometryType::MultiPoint(mp) => line_string(mp),
            GeometryType::LineString(ls) => Ok(ls.to_line_string()),
            GeometryType::MultiLineString(mls) => only(mls.line_strings())
                .map(|ls| ls.to_line_string())
                .ok_or(GeometryError::type_mismatch("LineString", &source)),
            GeometryType::Polygon(poly) => only_ring(poly)
                .map(|ring| ring.to_line_string())
                .ok_or(GeometryError::type_mismatch("LineString", &source)),
            _ => Err(GeometryError::type_mismatch("LineString", &source)),
        }
    }
}

impl DeserializeGeometry for geo_types::MultiLineString {
    fn deserialize_geometry<T: GeometryTrait<T = f64>>(source: T) -> Result<Self, GeometryError> {
        match source.as_type() {
            GeometryType::MultiPoint(mp) => {
                Ok(geo_types::MultiLineString::new(vec![line_string(mp)?]))
            }
            GeometryType::LineString(ls) => {
                Ok(geo_types::MultiLineString::new(vec![ls.to_line_string()]))
            }
            GeometryType::MultiLineString(mls) => Ok(mls.to_multi_line_string()),
            GeometryType::Polygon(poly) => {
                let mut lines = Vec::new();
                if let Some(ext) = poly.exterior() {
                    lines.push(ext.to_line_string());
                }
                for interior in poly.interiors() {
                    lines.push(interior.to_line_string());
                }
                Ok(geo_types::MultiLineString::new(lines))
            }
            _ => Err(GeometryError::type_mismatch("MultiLineString", &source)),
        }
    }
}

impl DeserializeGeometry for geo_types::Polygon {
    fn deserialize_geometry<T: GeometryTrait<T = f64>>(source: T) -> Result<Self, GeometryError> {
        match source.as_type() {
            GeometryType::MultiPoint(mp) => Ok(geo_types::Polygon::new(line_string(mp)?, vec![])),
            GeometryType::LineString(ls) => {
                Ok(geo_types::Polygon::new(ls.to_line_string(), vec![]))
            }
            GeometryType::MultiLineString(mls) => {
                let mut lines = mls.line_strings().map(|ls| ls.to_line_string());
                let exterior = lines
                    .next()
                    .unwrap_or_else(|| geo_types::LineString::new(vec![]));
                Ok(geo_types::Polygon::new(exterior, lines.collect()))
            }
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

fn line_string<M: MultiPointTrait<T = f64>>(
    mp: &M,
) -> Result<geo_types::LineString, GeometryError> {
    mp.points().map(|p| point(&p).map(|p| p.0)).collect()
}

/// Returns the item if `iter` has exactly one, so that flattening never drops
/// the rest.
fn only<I: Iterator>(mut iter: I) -> Option<I::Item> {
    let item = iter.next()?;
    iter.next().is_none().then_some(item)
}

/// Returns the exterior if `poly` has no interiors.
fn only_ring<P: PolygonTrait>(poly: &P) -> Option<P::RingType<'_>> {
    if poly.num_interiors() > 0 {
        return None;
    }
    poly.exterior()
}
