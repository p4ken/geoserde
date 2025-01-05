use geo_types::Point;
use geoserde::Feature;
use serde::{ser::SerializeStruct, Serialize};

fn main() {}

#[derive(Serialize, Feature)]
pub struct Child2 {
    #[geometry]
    loc: Point, // ←これはプロパティにならない
    count: i32, // ←これはプロパティ
}
pub struct Child2Feature {
    pub geometry: Point,
    pub properties: Child2Properties,
}
pub struct Child2Properties {
    pub count: i32,
}
impl geoserde::feature::SerializeFeature for Child2 {
    fn serialize_geometry<S, C>(&self, ser: S) -> Result<(), S::Error>
    where
        S: geoserde::feature::GeometrySerializer<C>,
        C: geo_types::CoordNum,
    {
        self.loc.serialize_geometry(ser)
        // ser.serialize_point(&self.loc)
    }

    fn serialize_properties<'a, S>(
        &self,
        ser: &'a mut S,
    ) -> Result<<&'a mut S as serde::Serializer>::Ok, <&'a mut S as serde::Serializer>::Error>
    where
        &'a mut S: serde::Serializer,
    {
        #[derive(Serialize)]
        struct Properties<'a> {
            count: &'a i32,
        }
        Properties { count: &self.count }.serialize(ser)
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
impl geoserde::feature::SerializeFeature for MyFeature2 {
    fn serialize_geometry<S, C>(&self, ser: S) -> Result<(), S::Error>
    where
        S: geoserde::feature::GeometrySerializer<C>,
        C: geo_types::CoordNum,
    {
        // serde(flatten) だから
        self.child.serialize_geometry(ser)
    }

    fn serialize_properties<'a, S>(
        &self,
        //&'a mut Sでも複数回呼べるわけではない
        ser: &'a mut S,
    ) -> Result<<&'a mut S as serde::Serializer>::Ok, <&'a mut S as serde::Serializer>::Error>
    where
        &'a mut S: serde::Serializer,
    {
        // serde(flatten) だから
        self.child.serialize_properties(ser)
        // moved error. Properties構造体が上とここで2回出てしまう問題もある
        // #[derive(Serialize)]
        // struct Properties<'a> {
        //     loc: &'a Point,
        //     title: &'a String,
        // }
        // Properties {
        //     loc: &self.loc,
        //     title: &self.title,
        // }
        // .serialize(ser)
    }
}

// データ形式寄りの、データ構造
pub struct MyFeature2_ {
    pub geometry: Point,
    pub properties: MyFeature2Properties,
}
pub struct MyFeature2Properties {
    pub title: String,
    pub count: i32,
}
