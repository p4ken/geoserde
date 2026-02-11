struct MyChild {
    // geoserde::Geometry
    geom: geo_types::Point,
    text: String,
}
impl MyChild {
    fn deserialize(de: FgbDeserializer) -> Self {
        de.deserialize_child()
    }
}

struct MyFeature {
    child: MyChild,
    number: i32,
}
impl MyFeature {
    fn deserialize(de: FgbDeserializer) -> Self {
        de.deserialize_feature()
    }
}

struct FgbDeserializer {
    seq_delim: &'static str,
    path_delim: &'static str,
}
impl FgbDeserializer {
    fn deserialize_child(&self) -> MyChild {
        todo!()
    }
    fn deserialize_feature(&self) -> MyFeature {
        todo!()
    }
    fn deserialize_geometry(&self) -> geo_types::Point {
        Default::default()
    }
    fn deserialize_i32(&self) -> i32 {
        0
    }
}
