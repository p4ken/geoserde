// TODO: ネストしたクラス。geojsonはany objectだから明示的にflattenが必要。

use geo_types::LineString;
use geoserde::{DeserializeFeature, GeoDeserialize};
use serde::Deserialize;

struct Parent {
    wrapped_geometry: Child,
    property: i32,
}
impl DeserializeFeature for Parent {
    fn deserialize_feature(fmt: impl geoserde::ParseFeature) -> Self {
        let wrapped_geometry = Child::deserialize_feature(fmt);
        let ((), p) = fmt.parse_feature();
        Self {
            wrapped_geometry,
            property: p,
        }
    }
}


#[derive(GeoDeserialize)]
struct Child {
    shape: LineString,
    property: bool,
}
