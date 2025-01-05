use geo_types::{Geometry, GeometryCollection, Line, LineString, Point, Polygon};

fn main() {}

pub trait GeometryFormat: Sized {
    fn point(self, point: &Point);
    fn line(self, line: &Line);
    fn line_string(self, line_string: &LineString);
    fn polygon(self, polygon: &Polygon);
    fn geometry(self, geometry: &Geometry);
    fn geometry_collection(self, geometry_collection: &GeometryCollection);
}

pub trait SerializeGeometry {
    fn serialize_geometry<S>(&self, serializer: S)
    where
        S: GeometryFormat;
}
