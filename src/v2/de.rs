pub mod fmt;
pub mod obj;

use geo_traits::GeometryTrait;
use serde::{de::DeserializeOwned, Deserializer};

pub trait DeserializeFeature: Sized {
    fn deserialize_feature(fmt: impl ParseFeature) -> Self;
}

pub trait DeserializeGeometry: Sized {
    // TODO: Result<Self>
    fn deserialize_geometry(fmt: impl GeometryTrait<T = f64>) -> Self;
}
impl DeserializeGeometry for () {
    fn deserialize_geometry(_: impl GeometryTrait<T = f64>) -> Self {
        ()
    }
}

pub trait DeserializeProperties: Sized {
    fn deserialize_properties<'de>(fmt: impl Deserializer<'de>) -> Self;
}
impl<T: DeserializeOwned> DeserializeProperties for T {
    fn deserialize_properties<'de>(fmt: impl Deserializer<'de>) -> Self {
        Self::deserialize(fmt).unwrap()
    }
}

pub trait ParseFeature {
    // P は serde::DeserializeOwned でも同じだが geometry との一貫性のため。
    fn parse_feature<G: DeserializeGeometry, P: DeserializeProperties>(self) -> (G, P);
}

// pub trait ParseProperty {
//     // 同じキーで複数回呼ばれるかもしれない（入れ子で同名フィールド）
//     fn parse_i32(&self, key: &str) -> i32;
// }

#[cfg(test)]
mod tests {
    use super::*;
    use geo_types::{Geometry, Point};

    #[derive(serde::Deserialize)]
    struct Properties {
        name: String,
        count: i32,
    }

    // TODO: ネストしたクラス。geojsonはany objectだから明示的にflattenが必要。

    #[test]
    fn test_deserialize_geometry() {
        let geom = Geometry::Point(Point::new(1.0, 2.0));
        let deserialized: Point = DeserializeGeometry::deserialize_geometry(geom);
        assert_eq!(deserialized.x(), 1.0);
        assert_eq!(deserialized.y(), 2.0);
    }
}
