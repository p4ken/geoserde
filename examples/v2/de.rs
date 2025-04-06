use geo_types::LineString;
use geoserde::v2::de::{DeserializeFeature, ParseFeature};
use geoserde_derive::Feature;
use serde::Deserialize;

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
