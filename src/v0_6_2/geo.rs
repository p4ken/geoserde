#![cfg(feature = "geo")]

use crate::v0_6_2::de::DeserializeGeometry;

impl DeserializeGeometry for geo_types::Point {
    fn deserialize_geometry<T: geo_traits::GeometryTrait<T = f64>>(source: T) -> Self {
        geo_traits::to_geo::ToGeoGeometry::try_to_geometry(&source)
            .unwrap()
            .try_into()
            .unwrap()
    }
}

impl DeserializeGeometry for geo_types::LineString {
    fn deserialize_geometry<T: geo_traits::GeometryTrait<T = f64>>(source: T) -> Self {
        geo_traits::to_geo::ToGeoGeometry::try_to_geometry(&source)
            .unwrap()
            .try_into()
            .unwrap()
    }
}
