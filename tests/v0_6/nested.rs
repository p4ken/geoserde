// TODO: ネストしたクラス。geojsonはany objectだから明示的にflattenが必要。

use geo_types::LineString;
use geoserde::{DeserializeFeature, GeoDeserialize};
use serde::Deserialize;

struct Parent {
    // #[geometry]
    wrapped_geometry: Child,
    property: i32,
}
impl DeserializeFeature for Parent {
    fn deserialize_feature(fmt: impl geoserde::ParseFeature) -> Self {
        let wrapped_geometry = Child::deserialize_feature(fmt);
        // ここのエラーは致命的。
        // serde の flatten のようなことをするには、
        // かなり大掛かりで serde そっくりの処理が必要になる。
        // いっそのこと serde に寄せて serde(with) などで解決できないか考えることに。
        let ((), p) = fmt.parse_feature();
        Self {
            wrapped_geometry,
            property: p,
        }
    }
}

#[derive(GeoDeserialize)]
struct Child {
    #[geometry]
    shape: LineString,
    property: bool,
}
