use geoserde::Feature;
use serde::Serialize;

#[derive(Serialize, Feature)]
pub struct Child1 {
    #[serde(with = "geometry")]
    loc: geo_types::Point,
    count: i32,
}

#[derive(Serialize)]
pub struct MyFeature1 {
    child: Child1,
    title: String,
}

pub mod geometry {
    /// For `serde(with=...)`
    pub fn serialize<S: serde::Serializer>(
        point: &geo_types::Point,
        ser: S,
    ) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        // 普通のフィールド名と被らない名前
        let mut state = ser.serialize_struct("__geoserde_geometry", 2)?;
        state.serialize_field("__geoserde_geometry_field", point)?;
        state.end()
    }
}

fn main() {}
