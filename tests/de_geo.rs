#![cfg(feature = "geo")]

use geo_types::{
    LineString, MultiLineString, MultiPoint, MultiPolygon, Point, Polygon, coord, line_string,
};
use geoserde::de::DeserializeGeometry;

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

// --- Converting without dropping coordinates ---

#[test]
fn point_to_point() {
    let p = Point::new(1., 2.);
    assert_eq!(Point::deserialize_geometry(p).unwrap(), p);
}

#[test]
fn point_to_multi_point() {
    let expected = MultiPoint::new(vec![Point::new(1., 2.)]);
    assert_eq!(
        MultiPoint::deserialize_geometry(Point::new(1., 2.)).unwrap(),
        expected
    );
}

#[test]
fn multi_point_to_multi_point() {
    let mp: MultiPoint = ring().points().collect();
    assert_eq!(MultiPoint::deserialize_geometry(&mp).unwrap(), mp);
}

#[test]
fn line_string_to_multi_point() {
    let expected: MultiPoint = ring().points().collect();
    assert_eq!(MultiPoint::deserialize_geometry(ring()).unwrap(), expected);
}

#[test]
fn multi_point_to_line_string() {
    let mp: MultiPoint = ring().points().collect();
    assert_eq!(LineString::deserialize_geometry(mp).unwrap(), ring());
}

#[test]
fn line_string_to_line_string() {
    assert_eq!(LineString::deserialize_geometry(ring()).unwrap(), ring());
}

#[test]
fn multi_point_to_multi_line_string() {
    let mp: MultiPoint = ring().points().collect();
    let expected = MultiLineString::new(vec![ring()]);
    assert_eq!(MultiLineString::deserialize_geometry(mp).unwrap(), expected);
}

#[test]
fn line_string_to_multi_line_string() {
    let expected = MultiLineString::new(vec![ring()]);
    assert_eq!(
        MultiLineString::deserialize_geometry(ring()).unwrap(),
        expected
    );
}

#[test]
fn multi_line_string_to_multi_line_string() {
    assert_eq!(
        MultiLineString::deserialize_geometry(two_lines()).unwrap(),
        two_lines()
    );
}

#[test]
fn donut_to_multi_line_string() {
    let donut = donut();
    let expected = MultiLineString::new(
        std::iter::once(donut.exterior().clone())
            .chain(donut.interiors().iter().cloned())
            .collect(),
    );
    assert_eq!(
        MultiLineString::deserialize_geometry(&donut).unwrap(),
        expected
    );
}

#[test]
fn empty_polygon_to_multi_line_string() {
    let poly = Polygon::new(LineString::new(vec![]), vec![]);
    assert_eq!(
        MultiLineString::deserialize_geometry(poly).unwrap(),
        MultiLineString::new(vec![])
    );
}

/// The ring is closed by `Polygon::new`.
#[test]
fn multi_point_to_polygon() {
    let mp: MultiPoint = ring().points().take(3).collect();
    let expected = Polygon::new(ring(), vec![]);
    assert_eq!(Polygon::deserialize_geometry(mp).unwrap(), expected);
}

/// The ring is closed by `Polygon::new`.
#[test]
fn line_string_to_polygon() {
    let open = line_string![(x: 0., y: 0.), (x: 1., y: 0.), (x: 1., y: 1.)];
    let expected = Polygon::new(ring(), vec![]);
    assert_eq!(Polygon::deserialize_geometry(open).unwrap(), expected);
}

#[test]
fn multi_line_string_to_polygon() {
    let donut = donut();
    let mls = MultiLineString::new(
        std::iter::once(donut.exterior().clone())
            .chain(donut.interiors().iter().cloned())
            .collect(),
    );
    assert_eq!(Polygon::deserialize_geometry(mls).unwrap(), donut);
}

#[test]
fn polygon_to_polygon() {
    assert_eq!(Polygon::deserialize_geometry(donut()).unwrap(), donut());
}

// --- Flattening a single element ---

#[test]
fn single_multi_point_to_point() {
    let mp = MultiPoint::new(vec![Point::new(1., 2.)]);
    assert_eq!(
        Point::deserialize_geometry(&mp).unwrap(),
        Point::new(1., 2.)
    );
}

#[test]
fn single_multi_line_string_to_line_string() {
    let mls = MultiLineString::new(vec![ring()]);
    assert_eq!(LineString::deserialize_geometry(&mls).unwrap(), ring());
}

#[test]
fn single_multi_line_string_to_multi_point() {
    let mls = MultiLineString::new(vec![ring()]);
    let expected: MultiPoint = ring().points().collect();
    assert_eq!(MultiPoint::deserialize_geometry(&mls).unwrap(), expected);
}

#[test]
fn simple_polygon_to_line_string() {
    let poly = Polygon::new(ring(), vec![]);
    assert_eq!(LineString::deserialize_geometry(&poly).unwrap(), ring());
}

#[test]
fn simple_polygon_to_multi_point() {
    let poly = Polygon::new(ring(), vec![]);
    let expected: MultiPoint = ring().points().collect();
    assert_eq!(MultiPoint::deserialize_geometry(&poly).unwrap(), expected);
}

// --- Rejected instead of dropping coordinates ---

#[test]
fn multi_point_to_point_rejected() {
    let mp = MultiPoint::new(vec![Point::new(1., 2.), Point::new(3., 4.)]);
    let err = Point::deserialize_geometry(&mp).unwrap_err();
    assert_eq!(err.to_string(), "expected Point, found MultiPoint");
}

#[test]
fn line_string_to_point_rejected() {
    let ls = LineString::new(vec![coord! { x: 1., y: 2. }]);
    let err = Point::deserialize_geometry(&ls).unwrap_err();
    assert_eq!(err.to_string(), "expected Point, found LineString");
}

#[test]
fn multi_line_string_to_point_rejected() {
    let mls = MultiLineString::new(vec![ring()]);
    let err = Point::deserialize_geometry(&mls).unwrap_err();
    assert_eq!(err.to_string(), "expected Point, found MultiLineString");
}

#[test]
fn polygon_to_point_rejected() {
    let poly = Polygon::new(ring(), vec![]);
    let err = Point::deserialize_geometry(&poly).unwrap_err();
    assert_eq!(err.to_string(), "expected Point, found Polygon");
}

#[test]
fn multi_line_string_to_line_string_rejected() {
    let err = LineString::deserialize_geometry(two_lines()).unwrap_err();
    assert_eq!(
        err.to_string(),
        "expected LineString, found MultiLineString"
    );
}

#[test]
fn multi_line_string_to_multi_point_rejected() {
    let err = MultiPoint::deserialize_geometry(two_lines()).unwrap_err();
    assert_eq!(
        err.to_string(),
        "expected MultiPoint, found MultiLineString"
    );
}

#[test]
fn donut_to_line_string_rejected() {
    let err = LineString::deserialize_geometry(donut()).unwrap_err();
    assert_eq!(err.to_string(), "expected LineString, found Polygon");
}

#[test]
fn donut_to_multi_point_rejected() {
    let err = MultiPoint::deserialize_geometry(donut()).unwrap_err();
    assert_eq!(err.to_string(), "expected MultiPoint, found Polygon");
}

#[test]
fn empty_multi_line_string_to_line_string_rejected() {
    let mls = MultiLineString::new(vec![]);
    let err = LineString::deserialize_geometry(&mls).unwrap_err();
    assert_eq!(
        err.to_string(),
        "expected LineString, found MultiLineString"
    );
}

#[test]
fn empty_polygon_to_line_string_rejected() {
    let poly = Polygon::new(LineString::new(vec![]), vec![]);
    let err = LineString::deserialize_geometry(poly).unwrap_err();
    assert_eq!(err.to_string(), "expected LineString, found Polygon");
}

#[test]
fn empty_multi_line_string_to_polygon_rejected() {
    let mls = MultiLineString::new(vec![]);
    let err = Polygon::deserialize_geometry(mls).unwrap_err();
    assert_eq!(err.to_string(), "expected Polygon, found MultiLineString");
}

// --- Unsupported geometry types ---

#[test]
fn multi_polygon_to_multi_point_rejected() {
    let mp = MultiPolygon::new(vec![donut()]);
    let err = MultiPoint::deserialize_geometry(mp).unwrap_err();
    assert_eq!(err.to_string(), "expected MultiPoint, found MultiPolygon");
}

#[test]
fn point_to_line_string_rejected() {
    let err = LineString::deserialize_geometry(Point::new(1., 2.)).unwrap_err();
    assert_eq!(err.to_string(), "expected LineString, found Point");
}

#[test]
fn point_to_multi_line_string_rejected() {
    let err = MultiLineString::deserialize_geometry(Point::new(1., 2.)).unwrap_err();
    assert_eq!(err.to_string(), "expected MultiLineString, found Point");
}

#[test]
fn point_to_polygon_rejected() {
    let err = Polygon::deserialize_geometry(Point::new(1., 2.)).unwrap_err();
    assert_eq!(err.to_string(), "expected Polygon, found Point");
}
