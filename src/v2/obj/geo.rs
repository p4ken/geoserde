use geo_traits::{to_geo::ToGeoLineString, GeometryTrait, GeometryType};
use serde::de::IgnoredAny;

use crate::v2::de::{DeserializeFeature, DeserializeGeometry, ParseFeature};

// プロパティなしでジオメトリのみ
impl DeserializeFeature for geo_types::LineString {
    fn deserialize_feature(fmt: impl ParseFeature) -> Self {
        fmt.parse_feature::<_, IgnoredAny>().0
    }
}

impl DeserializeGeometry for geo_types::LineString {
    // 2D以外もサポートできる
    // 複数種類のジオメトリ（コレクション）はサポートしない
    fn deserialize_geometry(fmt: impl GeometryTrait<T = f64>) -> Self {
        match fmt.as_type() {
            GeometryType::LineString(x) => x.to_line_string(),
            // マルチラインで来るかもよ
            _ => todo!(),
        }
    }
}
