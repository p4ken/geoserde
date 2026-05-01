#![cfg(feature = "geo")]

use geo_traits::{
    GeometryTrait, GeometryType,
    to_geo::{ToGeoLineString, ToGeoPoint},
};

use crate::de::DeserializeGeometry;

// `ToGeoGeometry::try_to_geometry` を直接呼ばないのは、`impl GeometryTrait<T=f64>` を
// 経由した呼び出しが trait solver の再帰展開で overflow するため
// (rustc #128887 / georust/geo #1385)。
// 単一バリアントのみを `as_type()` で取り出して `to_point` / `to_line_string` を呼ぶ
// blanket impl は、その内部で `GeometryTrait` を辿らないので再帰しない。

impl DeserializeGeometry for geo_types::Point {
    fn deserialize_geometry<T: GeometryTrait<T = f64>>(source: T) -> Self {
        match source.as_type() {
            GeometryType::Point(p) => p.to_point(),
            _ => panic!("DeserializeGeometry for Point: source is not a Point"),
        }
    }
}

impl DeserializeGeometry for geo_types::LineString {
    fn deserialize_geometry<T: GeometryTrait<T = f64>>(source: T) -> Self {
        match source.as_type() {
            GeometryType::LineString(ls) => ls.to_line_string(),
            _ => panic!("DeserializeGeometry for LineString: source is not a LineString"),
        }
    }
}
