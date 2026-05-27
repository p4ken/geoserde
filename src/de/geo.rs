#![cfg(feature = "geo")]

use geo_traits::{
    CoordTrait, GeometryTrait, GeometryType, LineStringTrait, MultiLineStringTrait,
    MultiPointTrait, PolygonTrait,
    to_geo::{ToGeoLineString, ToGeoMultiLineString, ToGeoMultiPoint, ToGeoPoint, ToGeoPolygon},
};

use crate::de::{DeserializeGeometry, GeometryTypeMismatch};

// `as_type()` で取り出した個別バリアントに `ToGeoPoint` 等の個別 trait を呼ぶ方式は
// `GeometryTrait` を辿らないため、`ToGeoGeometry` で起きる trait solver overflow
// (rustc #128887) の影響を受けない。
impl DeserializeGeometry for geo_types::Point {
    fn deserialize_geometry<T: GeometryTrait<T = f64>>(
        source: T,
    ) -> Result<Self, GeometryTypeMismatch> {
        match source.as_type() {
            GeometryType::Point(p) => Ok(p.to_point()),
            GeometryType::MultiPoint(mp) => mp
                .points()
                .next()
                .map(|p| p.to_point())
                .ok_or(GeometryTypeMismatch::new("Point")),
            GeometryType::LineString(ls) => ls
                .coords()
                .next()
                .map(|c| geo_types::Point::new(c.x(), c.y()))
                .ok_or(GeometryTypeMismatch::new("Point")),
            GeometryType::MultiLineString(mls) => {
                let ls = mls
                    .line_strings()
                    .next()
                    .ok_or(GeometryTypeMismatch::new("Point"))?;
                ls.coords()
                    .next()
                    .map(|c| geo_types::Point::new(c.x(), c.y()))
                    .ok_or(GeometryTypeMismatch::new("Point"))
            }
            GeometryType::Polygon(poly) => {
                let ring = poly.exterior().ok_or(GeometryTypeMismatch::new("Point"))?;
                ring.coords()
                    .next()
                    .map(|c| geo_types::Point::new(c.x(), c.y()))
                    .ok_or(GeometryTypeMismatch::new("Point"))
            }
            _ => Err(GeometryTypeMismatch::new("Point")),
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
                let ls = mls
                    .line_strings()
                    .next()
                    .ok_or(GeometryTypeMismatch::new("MultiPoint"))?;
                Ok(geo_types::MultiPoint::new(
                    ls.coords()
                        .map(|c| geo_types::Point::new(c.x(), c.y()))
                        .collect(),
                ))
            }
            GeometryType::Polygon(poly) => {
                let ring = poly
                    .exterior()
                    .ok_or(GeometryTypeMismatch::new("MultiPoint"))?;
                Ok(geo_types::MultiPoint::new(
                    ring.coords()
                        .map(|c| geo_types::Point::new(c.x(), c.y()))
                        .collect(),
                ))
            }
            _ => Err(GeometryTypeMismatch::new("MultiPoint")),
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
            GeometryType::MultiLineString(mls) => mls
                .line_strings()
                .next()
                .map(|ls| ls.to_line_string())
                .ok_or(GeometryTypeMismatch::new("LineString")),
            GeometryType::Polygon(poly) => poly
                .exterior()
                .map(|ring| ring.to_line_string())
                .ok_or(GeometryTypeMismatch::new("LineString")),
            _ => Err(GeometryTypeMismatch::new("LineString")),
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
            _ => Err(GeometryTypeMismatch::new("MultiLineString")),
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
                let mut lines: Vec<geo_types::LineString> =
                    mls.line_strings().map(|ls| ls.to_line_string()).collect();
                if lines.is_empty() {
                    return Err(GeometryTypeMismatch::new("Polygon"));
                }
                let exterior = lines.remove(0);
                Ok(geo_types::Polygon::new(exterior, lines))
            }
            GeometryType::Polygon(p) => Ok(p.to_polygon()),
            _ => Err(GeometryTypeMismatch::new("Polygon")),
        }
    }
}
