use std::marker::PhantomData;

use serde::{
    de::{DeserializeOwned, MapAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};

#[derive(Serialize, Deserialize)]
struct Child1 {
    #[serde(with = "geometry")]
    loc: geo_types::Point,
    count: i32,
}

#[derive(Serialize)]
struct MyFeature1 {
    child: Child1,
    title: String,
}

pub mod geometry {
    use geo_traits::{CoordTrait, PointTrait};
    use serde::{ser::SerializeStruct, Deserialize, Serialize};

    pub fn serialize<S: serde::Serializer>(
        geom: impl super::SerializeGeometry,
        ser: S,
    ) -> Result<S::Ok, S::Error> {
        super::__GeoSerdeGeometry(geom).serialize(ser)
    }

    // pub fn serialize0<S: serde::Serializer>(
    //     source: impl geo_traits::GeometryTrait<T: Serialize>,
    //     ser: S,
    // ) -> Result<S::Ok, S::Error> {
    //     match source.as_type() {
    //         geo_traits::GeometryType::Point(point) => {
    //             // 普通の構造体名と被らないname
    //             let mut point_ser = ser.serialize_struct("geoserde::Point", 2).unwrap();
    //             let coord = point.coord().unwrap();
    //             point_ser
    //                 .serialize_field("geoserde::x", &coord.x())
    //                 .unwrap();
    //             point_ser
    //                 .serialize_field("geoserde::y", &coord.y())
    //                 .unwrap();
    //             point_ser.end()
    //         }
    //         _ => todo!(),
    //     }
    // }
    pub fn deserialize<'a, D: serde::Deserializer<'a>, G: super::DeserializeGeometry>(
        de: D,
    ) -> Result<G, D::Error> {
        Ok(super::__GeoSerdeGeometry::deserialize(de)?.0)
    }
}

pub trait SerializeGeometry : Serialize {}
impl SerializeGeometry for geo_types::Point {}
impl<T: SerializeGeometry> SerializeGeometry for &T {}

// TODO: sealed
pub trait DeserializeGeometry : DeserializeOwned {}
impl DeserializeGeometry for geo_types::Point {}

// Wrapper to tell "this is the geometry" for data formats.
#[derive(Serialize, Deserialize)]
struct __GeoSerdeGeometry<T>(T);

// pub trait DeserializeGeometry0: Sized {
//     fn deserialize_geometry<'a, D: serde::Deserializer<'a>>(de: D) -> Result<Self, D::Error>;
// }
// impl DeserializeGeometry0 for geo_types::Point {
//     fn deserialize_geometry<'a, D: serde::Deserializer<'a>>(de: D) -> Result<Self, D::Error> {
//         // The name prevents collisions with real structs.
//         de.deserialize_newtype_struct("geoserde::Geometry", GeometryVisitor::default())
//     }
// }

// #[derive(Default)]
// struct GeometryVisitor<T>(PhantomData<T>);
// impl<'de, T> Visitor<'de> for GeometryVisitor<T> {
//     type Value = T;

//     fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
//         todo!()
//     }
// }

fn main() {}
