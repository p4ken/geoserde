use std::borrow::Cow;

use geo_traits::{
    Dimensions, GeometryTrait, GeometryType, UnimplementedLine, UnimplementedMultiLineString,
    UnimplementedMultiPoint, UnimplementedMultiPolygon, UnimplementedPoint, UnimplementedPolygon,
    UnimplementedRect, UnimplementedTriangle,
};
use geoserde::v0_6_2::fgb;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Feat {
    name: Cow<'static, str>,
    child: Child,
}

impl Feat {
    fn serialize(&self, ser: &mut fgb::FeatureSerializer) -> Result<(), fgb::ser::Error> {
        ser.serialize_feature(&self.child, &self)
    }
}

#[derive(Serialize, Deserialize)]
struct Child {
    value: i32,
    #[serde(skip, default = "geo_types::LineString::empty", rename = "geometry")]
    shape: geo_types::LineString,
}

impl<'a> GeometryTrait for &'a Child {
    type T = f64;
    type PointType<'b>
        = UnimplementedPoint<f64>
    where
        Self: 'b;
    type LineStringType<'b>
        = geo_types::LineString<f64>
    where
        Self: 'b;
    type PolygonType<'b>
        = UnimplementedPolygon<f64>
    where
        Self: 'b;
    type MultiPointType<'b>
        = UnimplementedMultiPoint<f64>
    where
        Self: 'b;
    type MultiLineStringType<'b>
        = UnimplementedMultiLineString<f64>
    where
        Self: 'b;
    type MultiPolygonType<'b>
        = UnimplementedMultiPolygon<f64>
    where
        Self: 'b;
    type GeometryCollectionType<'b>
        = geo_types::GeometryCollection<f64>
    where
        Self: 'b;
    type RectType<'b>
        = UnimplementedRect<f64>
    where
        Self: 'b;
    type TriangleType<'b>
        = UnimplementedTriangle<f64>
    where
        Self: 'b;
    type LineType<'b>
        = UnimplementedLine<f64>
    where
        Self: 'b;

    fn dim(&self) -> Dimensions {
        Dimensions::Xy
    }

    fn as_type(
        &self,
    ) -> GeometryType<
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
        GeometryType::LineString(&self.shape)
    }
}

#[test]
#[ignore = "WIP"]
fn ser_test() {
    let feat = Feat {
        name: "Hello".into(),
        child: Child {
            shape: crate::testing::ls(0),
            value: 42,
        },
    };
}
