use serde::Serialize;

use crate::v0_6_1::SerializeGeometry;

impl From<geo_types::Coord> for crate::v0_6_1::Point {
    fn from(source: geo_types::Coord) -> Self {
        Self {
            x: source.x,
            y: source.y,
            z: None,
            m: None,
        }
    }
}

impl SerializeGeometry for geo_types::Point {
    fn serialize_geometry<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        crate::v0_6_1::Point::from(self.0).serialize(ser)
    }
}
