fn main() {}

use geoserde::Feature;

// use geozero::FeatureAccess as FeatureType;
// use geozero::FeatureProperties as PropertiesType;
// use geozero::GeozeroDatasource as LayerType;
// use geozero::GeozeroGeometry as GeometryType;

// use geozero::FeatureProcessor as LayerFormat;
// use geozero::GeomProcessor as GeometryFormat;

#[derive(Feature)]
pub struct Child2 {
    #[geometry]
    pub loc: geo_types::Point, // ←これはプロパティにならない
    pub count: i32, // ←これはプロパティ
}
impl geozero::GeozeroDatasource for Child2 {
    fn process<P: geozero::FeatureProcessor>(
        &mut self,
        processor: &mut P,
    ) -> geozero::error::Result<()> {
        // 外部から任意のタイミングでgeometryを取りたい
        // propertiesは一部に過ぎないかもしれないからbegin,endはこちらでは指定できない
        processor.geometry_begin().unwrap();
        processor.geometry_end().unwrap();
        processor.properties_begin().unwrap();
        processor.properties_end().unwrap();
        todo!()
    }
}

#[derive(Feature)]
pub struct MyFeature2 {
    // #[serde(flatten)]
    // #[serde(skip)]
    // #[geometry]
    pub child: Child2,
    pub title: String,
}
