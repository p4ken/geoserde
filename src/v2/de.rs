use geo_traits::{to_geo::ToGeoLineString, GeometryTrait, GeometryType};
use geo_types::LineString;
use serde::de::DeserializeOwned;

pub trait DeserializeFeature: Sized {
    fn deserialize_feature(fmt: impl ParseFeature) -> Self;
}

pub trait DeserializeGeometry: Sized {
    fn deserialize_geometry(fmt: impl GeometryTrait<T = f64>) -> Self;
}
impl DeserializeGeometry for LineString {
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

pub trait ParseFeature {
    fn parse_feature<G: DeserializeGeometry, P: DeserializeOwned>(self) -> (G, P);
}

pub trait ParseProperty {
    // 同じキーで複数回呼ばれるかもしれない（入れ子で同名フィールド）
    fn parse_i32(&self, key: &str) -> i32;
}

#[cfg(test)]
mod tests {
    use geo_types::LineString;
    use geoserde_derive::Feature;
    use serde::Deserialize;

    use super::*;

    #[derive(Feature)]
    struct MyStruct {
        #[geometry]
        geom: LineString,
        prop_1: i32,
    }
    impl DeserializeFeature for MyStruct {
        fn deserialize_feature(fmt: impl ParseFeature) -> Self {
            #[derive(Deserialize)]
            struct Properties {
                prop_1: i32,
            }
            let (geom, properties) = fmt.parse_feature::<_, Properties>();

            Self {
                geom: geom,
                prop_1: properties.prop_1,
            }
        }
    }
}
