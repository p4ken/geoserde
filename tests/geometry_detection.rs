use std::{convert::Infallible, vec};

use geoserde::{GeometrySerializer, GeometrySink, SerializeError};
use serde::Serialize;

fn serialize_geometry(g: impl Serialize) -> (SinkMock, Result<(), SerializeError<Infallible>>) {
    let mut sink = SinkMock::default();
    let mut sut = GeometrySerializer::new(&mut sink);
    let ret = g.serialize(&mut sut);
    (sink, ret)
}

#[test]
fn none_test() {
    let g = Option::<()>::None;
    let (sink, ret) = serialize_geometry(g);
    let e = ret.unwrap_err();
    assert!(matches!(e, SerializeError::InvalidGeometryStructure { .. }));
    assert!(!sink.wrote_geometry)
}

#[test]
fn unit_test() {
    let g = ();
    let (sink, ret) = serialize_geometry(g);
    let e = ret.unwrap_err();
    assert!(matches!(e, SerializeError::InvalidGeometryStructure { .. }));
    assert!(!sink.wrote_geometry)
}

#[test]
fn tuple_test() {
    let g = (geo_types::Point::new(0.0, 0.1),);
    let (sink, ret) = serialize_geometry(g);
    let e = ret.unwrap_err();
    assert!(matches!(e, SerializeError::InvalidGeometryStructure { .. }));
    assert!(!sink.wrote_geometry)
}

#[test]
fn array_test() {
    let g = [geo_types::Point::new(0.0, 0.1)];
    let (sink, ret) = serialize_geometry(g);
    let e = ret.unwrap_err();
    assert!(matches!(e, SerializeError::InvalidGeometryStructure { .. }));
    assert!(!sink.wrote_geometry)
}

#[test]
fn vec_test() {
    let g = vec![geo_types::Point::new(0.0, 0.1)];
    let (sink, ret) = serialize_geometry(g);
    let e = ret.unwrap_err();
    assert!(matches!(e, SerializeError::InvalidGeometryStructure { .. }));
    assert!(!sink.wrote_geometry)
}

#[test]
fn point_test() {
    let g = geo_types::Point::new(0.0, 0.1);
    let (sink, ret) = serialize_geometry(g);
    assert!(ret.is_ok());
    assert!(sink.wrote_geometry);
}

#[test]
fn line_test() {
    let g = line_0();
    let (sink, ret) = serialize_geometry(g);
    assert!(ret.is_ok());
    assert!(sink.wrote_geometry);
}

#[test]
fn linestring_test() {
    let g = linestring_0();
    let (sink, ret) = serialize_geometry(g);
    assert!(ret.is_ok());
    assert!(sink.wrote_geometry);
}

#[test]
fn polygon_test() {
    let g = polygon_0();
    let (sink, ret) = serialize_geometry(g);
    assert!(ret.is_ok());
    assert!(sink.wrote_geometry);
}

#[test]
fn emptry_linestring_test() {
    let g = empty_linestring();
    let (sink, ret) = serialize_geometry(g);
    assert!(ret.is_ok());
    assert!(sink.wrote_geometry);
}

#[test]
fn emptry_polygon_test() {
    let g = emptry_polygon();
    let (sink, ret) = serialize_geometry(g);
    assert!(ret.is_ok());
    assert!(sink.wrote_geometry);
}

#[test]
fn no_field_coord_test() {
    #[derive(Serialize)]
    struct Coord {}
    let g = Coord {};
    let (sink, ret) = serialize_geometry(g);
    let e = ret.unwrap_err();
    assert!(matches!(e, SerializeError::InvalidGeometryStructure { .. }));
    assert!(!sink.wrote_geometry)
}

#[test]
fn no_field_point_test() {
    #[derive(Serialize)]
    struct Point();
    let g = Point {};
    let (sink, ret) = serialize_geometry(g);
    let e = ret.unwrap_err();
    assert!(matches!(e, SerializeError::InvalidGeometryStructure { .. }));
    assert!(!sink.wrote_geometry)
}

#[test]
fn no_field_line_test() {
    #[derive(Serialize)]
    struct Line {}
    let g = Line {};
    let (sink, ret) = serialize_geometry(g);
    let e = ret.unwrap_err();
    assert!(matches!(e, SerializeError::InvalidGeometryStructure { .. }));
    assert!(!sink.wrote_geometry)
}

#[test]
fn no_field_linestring_test() {
    #[derive(Serialize)]
    struct LineString();
    let g = LineString {};
    let (sink, ret) = serialize_geometry(g);
    let e = ret.unwrap_err();
    assert!(matches!(e, SerializeError::InvalidGeometryStructure { .. }));
    assert!(!sink.wrote_geometry)
}

#[test]
fn no_field_polygon_test() {
    #[derive(Serialize)]
    struct Polygon {}
    let g = Polygon {};
    let (sink, ret) = serialize_geometry(g);
    let e = ret.unwrap_err();
    assert!(matches!(e, SerializeError::InvalidGeometryStructure { .. }));
    assert!(!sink.wrote_geometry)
}

#[test]
fn fake_point_test() {
    let g = fake_point_0();
    let (sink, ret) = serialize_geometry(g);
    ret.unwrap_err();
    assert!(!sink.wrote_geometry)
}

#[test]
fn fake_line_test() {
    let g = fake_line_0();
    let (sink, ret) = serialize_geometry(g);
    assert!(ret.is_err());
    assert!(!sink.wrote_geometry)
}

#[test]
fn fake_linestring_test() {
    let g = fake_linestring_0();
    let (sink, ret) = serialize_geometry(g);
    assert!(ret.is_err());
    assert!(!sink.wrote_geometry)
}

#[test]
fn fake_polygon_test() {
    let g = fake_polygon_0();
    let (sink, ret) = serialize_geometry(g);
    assert!(ret.is_err());
    assert!(!sink.wrote_geometry)
}

#[derive(Serialize)]
struct Point {
    x: f64,
    y: f64,
}
fn fake_point_0() -> Point {
    let geo_types::Coord { x, y } = point_0().0;
    Point { x, y }
}
fn point_0() -> geo_types::Point {
    [0.0, 0.1].into()
}
fn point_1() -> geo_types::Point {
    [1.0, 1.1].into()
}

#[derive(Serialize)]
struct Line {
    start: f64,
    end: f64,
}
fn fake_line_0() -> Line {
    Line {
        start: 1.0,
        end: 2.0,
    }
}
fn line_0() -> geo_types::Line {
    geo_types::Line::new(point_0(), point_1())
}

#[derive(Serialize)]
struct LineString(Vec<f64>);
fn fake_linestring_0() -> LineString {
    LineString(vec![0.0])
}
fn linestring_0() -> geo_types::LineString {
    geo_types::LineString::new(vec![point_0().0, point_1().0])
}
fn empty_linestring() -> geo_types::LineString {
    geo_types::LineString::<f64>(vec![])
}

#[derive(Serialize)]
struct Polygon {
    exterior: LineString,
    interior: Vec<LineString>,
}
fn fake_polygon_0() -> Polygon {
    Polygon {
        exterior: fake_linestring_0(),
        interior: vec![],
    }
}
fn polygon_0() -> geo_types::Polygon {
    geo_types::Polygon::new(linestring_0(), vec![])
}
fn emptry_polygon() -> geo_types::Polygon {
    geo_types::Polygon::new(empty_linestring(), vec![])
}

#[derive(Debug, Default)]
struct SinkMock {
    wrote_geometry: bool,
}
impl SinkMock {
    fn write_geometry(&mut self) {
        self.wrote_geometry = true;
    }
}
impl GeometrySink for SinkMock {
    type Err = Infallible;
    fn coord(&mut self, _: usize, _: f64, _: f64) -> Result<(), Self::Err> {
        Ok(self.write_geometry())
    }
    fn point_start(&mut self, _: usize) -> Result<(), Self::Err> {
        Ok(self.write_geometry())
    }
    fn point_end(&mut self, _: usize) -> Result<(), Self::Err> {
        Ok(self.write_geometry())
    }
    fn linestring_start(&mut self, _: bool, _: usize, _: usize) -> Result<(), Self::Err> {
        Ok(self.write_geometry())
    }
    fn linestring_end(&mut self, _: bool, _: usize) -> Result<(), Self::Err> {
        Ok(self.write_geometry())
    }
    fn polygon_start(&mut self, _: bool, _: usize) -> Result<(), Self::Err> {
        Ok(self.write_geometry())
    }
    fn polygon_end(&mut self, _: bool, _: usize) -> Result<(), Self::Err> {
        Ok(self.write_geometry())
    }
    fn geometry_start(&mut self) -> Result<(), Self::Err> {
        Ok(self.write_geometry())
    }
    fn geometry_end(&mut self) -> Result<(), Self::Err> {
        Ok(self.write_geometry())
    }
}
