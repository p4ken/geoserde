use geo_types::LineString;
use geoserde_derive::Feature;

#[derive(Feature)]
struct MyStruct {
    #[geometry]
    my_geom: LineString,
    my_prop: i32,
}
impl geoserde::DeserializeFeature for MyStruct {
    fn deserialize_feature(fmt: impl geoserde::ParseFeature) -> Self {
        #[derive(geoserde::serde::Deserialize)]
        struct __Properties {
            my_prop: i32,
        }
        let (__geom, __props) = fmt.parse_feature::<_, __Properties>();
        Self {
            my_geom: __geom,
            my_prop: __props.my_prop,
        }
    }
}
