// 独自APIって要るの?
pub trait SerializePoint {
    // fn serialize_xy(&mut self, x: f64, y: f64);
    fn serialize_coordinate_value(&mut self, name: &'static str, value: f64);
    fn end(self);
}

pub trait SerializeLineString: SerializePoint {
    fn serialize_point<T>(&mut self, point: T)
    where
        T: ?Sized + SerializePoint;
    fn end(self);
}

pub trait SerializePolygon {
    fn serialize_exterior<T>(&mut self, exterior: T)
    where
        T: ?Sized + SerializeLineString;
    fn serialize_interior<T>(&mut self, interior: T)
    where
        T: ?Sized + SerializeLineString;
    fn end(self);
}
pub trait SerializeGeometricObject: Sized {
    type SerializeLineString: SerializeLineString;
    type SerializePolygon: SerializePolygon;

    fn serialize_point<T>(self, point: T)
    where
        T: ?Sized + SerializePoint;
    // fn serialize_line(self) -> Self::SerializeLineString {
    //     self.serialize_line_string(Some(2))
    // }
    fn serialize_line_string(self, len: Option<usize>) -> Self::SerializeLineString;
    fn serialize_polygon(self, rings_len: Option<usize>) -> Self::SerializePolygon;
}
pub trait SerializeGeometricObjectCollection: Sized {
    fn serialize_geometric_object<T>(&mut self, obj: T)
    where
        T: ?Sized + SerializeGeometricObject;
    fn end(self);
}

pub trait GeometrySerializer: Sized {
    type SerializeGeometricObjectCollection: SerializeGeometricObjectCollection;
    fn serialize_geometric_object<T>(self, obj: T)
    where
        T: ?Sized + SerializeGeometricObject,
    {
        let mut state = self.serialize_geometry_collection();
        state.serialize_geometric_object(obj);
        state.end()
    }
    fn serialize_geometry_collection(self) -> Self::SerializeGeometricObjectCollection;
}

// これは任意の構造体用
pub trait SerializeGeometry {
    fn serialize_geometry<S>(&self, serializer: S)
    where
        S: GeometrySerializer;
}

fn main() {}
