#![cfg(feature = "geo")]

use crate::v0_6_2::de::DeserializeGeometry;
use geo_traits::{CoordTrait, GeometryType, PointTrait};

impl DeserializeGeometry for geo_types::Point {
    fn deserialize_geometry<T: geo_traits::GeometryTrait<T = f64>>(source: T) -> Self {
        match source.as_type() {
            GeometryType::Point(p) => match p.coord() {
                Some(c) => geo_types::Point::new(c.x(), c.y()),
                None => panic!("deserialize_geometry: Point has no coordinate"),
            },
            _ => panic!("deserialize_geometry: expected Point geometry"),
        }
    }
}
