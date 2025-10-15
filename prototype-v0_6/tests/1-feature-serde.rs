use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Child2 {
    pub loc: geo_types::Point,
    pub count: i32,
    pub count2: i32,
}

pub struct MyFeature2 {
    pub child: Child2,
    pub title: String,
}

pub struct MyFormat;

pub trait Feature {
    // ジオメトリとプロパティのどちらを先にシリアライズするか、データ形式によって選べる
    fn serialize_geometry(&self, ser: &mut impl serde::Serializer);
    fn serialize_properties(&self, ser: impl serde::Serializer);
}
impl Feature for MyFeature2 {
    fn serialize_geometry(&self, ser: &mut impl serde::Serializer) {
        self.child.serialize_geometry(ser);
        // 無いなら
        self.title.serialize_geometry(ser);
        // 無いなら、 **データ形式次第では** エラー
        todo!()
    }

    fn serialize_properties(&self, ser: impl serde::Serializer) {
        // これだと #[serde(skip)] とかが効かない・・・
        // self.child.serialize_properties(ser);
        // self.title.serialize_properties(ser);

        #[derive(Serialize)]
        struct __Properties<'a> {
            // これだと loc もプロパティとして出力されてしまう！！
            pub child: &'a Child2,
            pub title: &'a String,
        }
        __Properties {
            child: &self.child,
            title: &self.title,
        }
        .serialize(ser)
        .unwrap();
    }
}
// impl Feature {
//     pub fn deserialize_properties(de: impl serde::Deserializer) {}
// }
impl Feature for Child2 {
    fn serialize_geometry(&self, _ser: &mut impl serde::Serializer) {
        todo!()
    }

    fn serialize_properties(&self, _ser: impl serde::Serializer) {
        // self.count.serialize(ser).unwrap();
        // self.count2.serialize(ser).unwrap(); // 所有権エラー。serは親子で使いまわせない。
    }
}
impl Feature for String {
    fn serialize_geometry(&self, _ser: &mut impl serde::Serializer) {
        todo!()
    }

    fn serialize_properties(&self, _ser: impl serde::Serializer) {
        todo!()
    }
}

fn main() {}
