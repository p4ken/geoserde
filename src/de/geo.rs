#![cfg(feature = "geo")]

use geo_traits::{
    CoordTrait, GeometryTrait, GeometryType, LineStringTrait, MultiLineStringTrait,
    MultiPointTrait, PolygonTrait,
    to_geo::{ToGeoLineString, ToGeoMultiLineString, ToGeoMultiPoint, ToGeoPoint, ToGeoPolygon},
};

use crate::de::{DeserializeGeometry, GeometryTypeMismatch};

// Calling per-variant traits such as `ToGeoPoint` on the variant taken out by `as_type()`
// does not go through `GeometryTrait`, so it avoids the trait solver overflow that
// `ToGeoGeometry` hits (rustc #128887).
impl DeserializeGeometry for geo_types::Point {
    fn deserialize_geometry<T: GeometryTrait<T = f64>>(
        source: T,
    ) -> Result<Self, GeometryTypeMismatch> {
        match source.as_type() {
            GeometryType::Point(p) => Ok(p.to_point()),
            GeometryType::MultiPoint(mp) => only(mp.points())
                .map(|p| p.to_point())
                .ok_or(GeometryTypeMismatch::new("Point", &source)),
            _ => Err(GeometryTypeMismatch::new("Point", &source)),
        }
    }
}

impl DeserializeGeometry for geo_types::MultiPoint {
    fn deserialize_geometry<T: GeometryTrait<T = f64>>(
        source: T,
    ) -> Result<Self, GeometryTypeMismatch> {
        match source.as_type() {
            GeometryType::Point(p) => Ok(geo_types::MultiPoint::new(vec![p.to_point()])),
            GeometryType::MultiPoint(mp) => Ok(mp.to_multi_point()),
            GeometryType::LineString(ls) => Ok(geo_types::MultiPoint::new(
                ls.coords()
                    .map(|c| geo_types::Point::new(c.x(), c.y()))
                    .collect(),
            )),
            GeometryType::MultiLineString(mls) => {
                let ls = only(mls.line_strings())
                    .ok_or(GeometryTypeMismatch::new("MultiPoint", &source))?;
                Ok(geo_types::MultiPoint::new(
                    ls.coords()
                        .map(|c| geo_types::Point::new(c.x(), c.y()))
                        .collect(),
                ))
            }
            GeometryType::Polygon(poly) => {
                let ring =
                    only_ring(poly).ok_or(GeometryTypeMismatch::new("MultiPoint", &source))?;
                Ok(geo_types::MultiPoint::new(
                    ring.coords()
                        .map(|c| geo_types::Point::new(c.x(), c.y()))
                        .collect(),
                ))
            }
            _ => Err(GeometryTypeMismatch::new("MultiPoint", &source)),
        }
    }
}

impl DeserializeGeometry for geo_types::LineString {
    fn deserialize_geometry<T: GeometryTrait<T = f64>>(
        source: T,
    ) -> Result<Self, GeometryTypeMismatch> {
        match source.as_type() {
            GeometryType::MultiPoint(mp) => Ok(geo_types::LineString::new(
                mp.points().map(|p| p.to_point().0).collect(),
            )),
            GeometryType::LineString(ls) => Ok(ls.to_line_string()),
            GeometryType::MultiLineString(mls) => only(mls.line_strings())
                .map(|ls| ls.to_line_string())
                .ok_or(GeometryTypeMismatch::new("LineString", &source)),
            GeometryType::Polygon(poly) => only_ring(poly)
                .map(|ring| ring.to_line_string())
                .ok_or(GeometryTypeMismatch::new("LineString", &source)),
            _ => Err(GeometryTypeMismatch::new("LineString", &source)),
        }
    }
}

impl DeserializeGeometry for geo_types::MultiLineString {
    fn deserialize_geometry<T: GeometryTrait<T = f64>>(
        source: T,
    ) -> Result<Self, GeometryTypeMismatch> {
        match source.as_type() {
            GeometryType::MultiPoint(mp) => Ok(geo_types::MultiLineString::new(vec![
                geo_types::LineString::new(mp.points().map(|p| p.to_point().0).collect()),
            ])),
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
            _ => Err(GeometryTypeMismatch::new("MultiLineString", &source)),
        }
    }
}

impl DeserializeGeometry for geo_types::Polygon {
    fn deserialize_geometry<T: GeometryTrait<T = f64>>(
        source: T,
    ) -> Result<Self, GeometryTypeMismatch> {
        match source.as_type() {
            GeometryType::MultiPoint(mp) => Ok(geo_types::Polygon::new(
                geo_types::LineString::new(mp.points().map(|p| p.to_point().0).collect()),
                vec![],
            )),
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
            _ => Err(GeometryTypeMismatch::new("Polygon", &source)),
        }
    }
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
