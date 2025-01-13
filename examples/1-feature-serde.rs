fn main() {}

use geoserde::Feature;

#[derive(Feature)]
pub struct Child2 {
    #[geometry]
    pub loc: geo_types::Point,
    pub count: i32,
    pub count2: i32,
}

#[derive(Feature)]
pub struct MyFeature2 {
    pub child: Child2,
    pub title: String,
}

pub struct MyFormat;

pub trait Feature {
    // ジオメトリとプロパティのどちらを先にシリアライズするか、データ形式によって選べる
    fn serialize_geometry(&self, ser: &mut impl serde::Serializer);
    fn serialize_properties(&self, ser: &mut impl serde::Serializer);
}
impl Feature for MyFeature2 {
    fn serialize_geometry(&self, ser: &mut impl serde::Serializer) {
        self.child.serialize_geometry(ser);
        // ダメなら
        self.title.serialize_geometry(ser);
        // ダメなら、エラー
    }

    fn serialize_properties(&self, ser: &mut impl serde::Serializer) {
        todo!()
    }
}
impl Feature for String {
    fn serialize_geometry(&self, ser: &mut impl serde::Serializer) {
        todo!()
    }

    fn serialize_properties(&self, ser: &mut impl serde::Serializer) {
        todo!()
    }
}
impl Feature for Child2 {
    fn serialize_geometry(&self, ser: &mut impl serde::Serializer) {
        todo!()
    }

    fn serialize_properties(&self, ser: &mut impl serde::Serializer) {
        // self.count.serialize(ser).unwrap();
        // self.count2.serialize(ser).unwrap(); // 所有権エラー だから 1d
    }
}
