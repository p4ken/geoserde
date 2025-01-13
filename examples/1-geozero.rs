use geo_types::Point;
use geoserde::Feature;
use serde::{ser::SerializeStruct, Serialize};

// use geozero::FeatureAccess as FeatureType;
// use geozero::FeatureProperties as PropertiesType;
// use geozero::GeozeroDatasource as LayerType;
// use geozero::GeozeroGeometry as GeometryType;

// use geozero::FeatureProcessor as LayerFormat;
// use geozero::GeomProcessor as GeometryFormat;

fn main() {}

#[derive(Serialize, Feature)]
pub struct Child2 {
    #[geometry]
    loc: Point, // ←これはプロパティにならない
    count: i32, // ←これはプロパティ
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

#[derive(Serialize, Feature)]
pub struct MyFeature2 {
    // #[serde(flatten)]
    // #[serde(skip)]
    // #[geometry]
    child: Child2,
    title: String,
}
impl MyFeature2 {
    pub fn serialize_properties<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // geozeroよりもserdeの方がAPIが安定しているので良い
        // serdeでやるなら、擬似的な構造体を作る
        let mut state = ser.serialize_struct("properties", 2)?;
        state.serialize_field("title", &self.title)?;
        state.serialize_field("child", &self.child)?; // Serializer側でstructはflattenされる
        state.end()
    }
    pub fn process_properties<P>(&self, pro: &mut P) -> Result<bool, geozero::error::GeozeroError>
    where
        P: geozero::PropertyProcessor,
    {
        // std::String かどうかはマクロでは分からないが
        pro.property(0, "title", &geozero::ColumnValue::String(&self.title))?;
        // child の中身はマクロでは分からないが
        pro.property(0, "count", &geozero::ColumnValue::Int(self.child.count))
    }
}
