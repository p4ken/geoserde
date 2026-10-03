#![cfg(feature = "geo-types")]

use geo_types::{
    LineString, MultiLineString, MultiPoint, MultiPolygon, Point, Polygon, line_string,
};
use geoserde::de::{DeserializeGeometry, GeometryOptions};

fn ring() -> LineString {
    line_string![(x: 0., y: 0.), (x: 1., y: 0.), (x: 1., y: 1.), (x: 0., y: 0.)]
}

fn donut() -> Polygon {
    let hole = line_string![(x: 0.2, y: 0.1), (x: 0.9, y: 0.1), (x: 0.9, y: 0.8), (x: 0.2, y: 0.1)];
    Polygon::new(ring(), vec![hole])
}

fn two_lines() -> MultiLineString {
    MultiLineString::new(vec![ring(), line_string![(x: 5., y: 5.), (x: 6., y: 6.)]])
}

fn promoting() -> GeometryOptions {
    let mut options = GeometryOptions::new();
    options.set_promote_to_multi(true);
    options
}

// --- Same type ---

#[test]
fn point_to_point() {
    let p = Point::new(1., 2.);
    assert_eq!(
        Point::deserialize_geometry(p, &GeometryOptions::new()).unwrap(),
        p
    );
}

#[test]
fn multi_point_to_multi_point() {
    let mp: MultiPoint = ring().points().collect();
    assert_eq!(
        MultiPoint::deserialize_geometry(&mp, &GeometryOptions::new()).unwrap(),
        mp
    );
}

#[test]
fn line_string_to_line_string() {
    assert_eq!(
        LineString::deserialize_geometry(ring(), &GeometryOptions::new()).unwrap(),
        ring()
    );
}

#[test]
fn multi_line_string_to_multi_line_string() {
    assert_eq!(
        MultiLineString::deserialize_geometry(two_lines(), &GeometryOptions::new()).unwrap(),
        two_lines()
    );
}

#[test]
fn polygon_to_polygon() {
    assert_eq!(
        Polygon::deserialize_geometry(donut(), &GeometryOptions::new()).unwrap(),
        donut()
    );
}

// --- Promoting to multi ---

#[test]
fn point_to_multi_point_rejected() {
    let err =
        MultiPoint::deserialize_geometry(Point::new(1., 2.), &GeometryOptions::new()).unwrap_err();
    assert_eq!(err.to_string(), "expected MultiPoint, found Point");
}

#[test]
fn point_to_multi_point_promoted() {
    let expected = MultiPoint::new(vec![Point::new(1., 2.)]);
    assert_eq!(
        MultiPoint::deserialize_geometry(Point::new(1., 2.), &promoting()).unwrap(),
        expected
    );
}

#[test]
fn line_string_to_multi_line_string_rejected() {
    let err = MultiLineString::deserialize_geometry(ring(), &GeometryOptions::new()).unwrap_err();
    assert_eq!(
        err.to_string(),
        "expected MultiLineString, found LineString"
    );
}

#[test]
fn line_string_to_multi_line_string_promoted() {
    let expected = MultiLineString::new(vec![ring()]);
    assert_eq!(
        MultiLineString::deserialize_geometry(ring(), &promoting()).unwrap(),
        expected
    );
}

// --- Demoting to single ---

/// `promote_to_multi` does not allow the opposite direction.
#[test]
fn single_multi_point_to_point_rejected() {
    let mp = MultiPoint::new(vec![Point::new(1., 2.)]);
    let err = Point::deserialize_geometry(&mp, &promoting()).unwrap_err();
    assert_eq!(err.to_string(), "expected Point, found MultiPoint");
}

#[test]
fn single_multi_line_string_to_line_string_rejected() {
    let mls = MultiLineString::new(vec![ring()]);
    let err = LineString::deserialize_geometry(&mls, &promoting()).unwrap_err();
    assert_eq!(
        err.to_string(),
        "expected LineString, found MultiLineString"
    );
}

// --- Crossing dimensions ---

#[test]
fn multi_point_to_line_string_rejected() {
    let mp: MultiPoint = ring().points().collect();
    let err = LineString::deserialize_geometry(&mp, &promoting()).unwrap_err();
    assert_eq!(err.to_string(), "expected LineString, found MultiPoint");
}

#[test]
fn line_string_to_multi_point_rejected() {
    let err = MultiPoint::deserialize_geometry(ring(), &promoting()).unwrap_err();
    assert_eq!(err.to_string(), "expected MultiPoint, found LineString");
}

#[test]
fn point_to_line_string_rejected() {
    let err = LineString::deserialize_geometry(Point::new(1., 2.), &promoting()).unwrap_err();
    assert_eq!(err.to_string(), "expected LineString, found Point");
}

#[test]
fn point_to_multi_line_string_rejected() {
    let err = MultiLineString::deserialize_geometry(Point::new(1., 2.), &promoting()).unwrap_err();
    assert_eq!(err.to_string(), "expected MultiLineString, found Point");
}

#[test]
fn line_string_to_polygon_rejected() {
    let err = Polygon::deserialize_geometry(ring(), &promoting()).unwrap_err();
    assert_eq!(err.to_string(), "expected Polygon, found LineString");
}

#[test]
fn multi_line_string_to_polygon_rejected() {
    let err = Polygon::deserialize_geometry(two_lines(), &promoting()).unwrap_err();
    assert_eq!(err.to_string(), "expected Polygon, found MultiLineString");
}

#[test]
fn polygon_to_line_string_rejected() {
    let poly = Polygon::new(ring(), vec![]);
    let err = LineString::deserialize_geometry(&poly, &promoting()).unwrap_err();
    assert_eq!(err.to_string(), "expected LineString, found Polygon");
}

#[test]
fn polygon_to_multi_line_string_rejected() {
    let err = MultiLineString::deserialize_geometry(donut(), &promoting()).unwrap_err();
    assert_eq!(err.to_string(), "expected MultiLineString, found Polygon");
}

#[test]
fn polygon_to_point_rejected() {
    let poly = Polygon::new(ring(), vec![]);
    let err = Point::deserialize_geometry(&poly, &promoting()).unwrap_err();
    assert_eq!(err.to_string(), "expected Point, found Polygon");
}

// --- Unsupported geometry types ---

#[test]
fn multi_polygon_to_polygon_rejected() {
    let mp = MultiPolygon::new(vec![donut()]);
    let err = Polygon::deserialize_geometry(mp, &promoting()).unwrap_err();
    assert_eq!(err.to_string(), "expected Polygon, found MultiPolygon");
}

// --- Empty points ---

#[test]
fn empty_point_to_point_rejected() {
    let err = Point::deserialize_geometry(EmptyPoint, &GeometryOptions::new()).unwrap_err();
    assert_eq!(err.to_string(), "geo-types cannot represent an empty point");
}

#[test]
fn empty_point_to_multi_point_rejected() {
    let err = MultiPoint::deserialize_geometry(EmptyPoint, &promoting()).unwrap_err();
    assert_eq!(err.to_string(), "geo-types cannot represent an empty point");
}

/// `geo_types::Point` cannot be empty, so the source implements `PointTrait` on its own.
struct EmptyPoint;

impl geo_traits::PointTrait for EmptyPoint {
    type CoordType<'a> = geo_types::Coord;

    fn coord(&self) -> Option<Self::CoordType<'_>> {
        None
    }
}

impl geo_traits::GeometryTrait for EmptyPoint {
    type T = f64;
    type PointType<'a> = EmptyPoint;
    type LineStringType<'a> = geo_traits::UnimplementedLineString<f64>;
    type PolygonType<'a> = geo_traits::UnimplementedPolygon<f64>;
    type MultiPointType<'a> = geo_traits::UnimplementedMultiPoint<f64>;
    type MultiLineStringType<'a> = geo_traits::UnimplementedMultiLineString<f64>;
    type MultiPolygonType<'a> = geo_traits::UnimplementedMultiPolygon<f64>;
    type GeometryCollectionType<'a> = geo_traits::UnimplementedGeometryCollection<f64>;
    type RectType<'a> = geo_traits::UnimplementedRect<f64>;
    type TriangleType<'a> = geo_traits::UnimplementedTriangle<f64>;
    type LineType<'a> = geo_traits::UnimplementedLine<f64>;

    fn dim(&self) -> geo_traits::Dimensions {
        geo_traits::Dimensions::Xy
    }

    fn as_type(
        &self,
    ) -> geo_traits::GeometryType<
        '_,
        Self::PointType<'_>,
        Self::LineStringType<'_>,
        Self::PolygonType<'_>,
        Self::MultiPointType<'_>,
        Self::MultiLineStringType<'_>,
        Self::MultiPolygonType<'_>,
        Self::GeometryCollectionType<'_>,
        Self::RectType<'_>,
        Self::TriangleType<'_>,
        Self::LineType<'_>,
    > {
        geo_traits::GeometryType::Point(self)
    }
}
