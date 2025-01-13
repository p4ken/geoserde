use geoserde::Feature;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Feature)]
pub struct Child1 {
    #[serde(with = "geometry")]
    loc: geo_types::Point,
    count: i32,
}

#[derive(Serialize)]
pub struct MyFeature1 {
    // #[serde(flatten)] // あっても無くても論理は同じ
    child: Child1,
    title: String,
}

pub mod geometry {
    /// For `serde(with=...)`
    pub fn serialize<S: serde::Serializer>(
        point: &geo_types::Point,
        ser: S,
    ) -> Result<S::Ok, S::Error> {
        // 普通のフィールド名と被らない名前
        ser.serialize_newtype_struct("__geoserde_geometry", point)
    }
    pub fn deserialize<'a, D: serde::Deserializer<'a>>(
        de: D,
    ) -> Result<geo_types::Point, D::Error> {
        // serde に寄せた旨みがない・・・
        struct __Visitor;
        impl<'b> serde::de::Visitor<'b> for __Visitor {
            type Value = geo_types::Point;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("map")
            }
            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'b>,
            {
                // ここのカスタマイズはserde_deriveじゃないと難しい
                todo!()
            }
        }
        let visitor = __Visitor;
        de.deserialize_newtype_struct("__geoserde_geometry", visitor)

        // 結局 visitor が要る
        // struct De<T>(T);
        // impl<'de, T: serde::Deserializer<'de>> serde::Deserializer<'de> for De<T> {
        //     type Error;
        //     fn deserialize_tuple_struct<V>(
        //         self,
        //         name: &'static str,
        //         len: usize,
        //         visitor: V,
        //     ) -> Result<V::Value, Self::Error>
        //     where
        //         V: serde::de::Visitor<'de>,
        //     {
        //         self.deserialize_tuple_struct("__geoserde_geometry", 1, visitor)
        //         todo!()
        //     }
        // }
        // serde::Deserialize::deserialize(de)
    }
}

fn main() {}
