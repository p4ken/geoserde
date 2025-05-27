use dbase::Record;
use geo_traits::{GeometryTrait, MultiLineStringTrait};
use geo_types::Geometry;
use serde::{de::DeserializeOwned, Deserializer};
use shapefile::{ReadableShape, Shape, ShapeReader};

use crate::v2::de::{DeserializeGeometry, DeserializeProperties, ParseFeature};

// impl<R0, R1> ParseLayer for shapefile::Reader<R0, R1> {}

// // もはや deserialize もない
// impl ParseFeature for (Shape, Record) {
//     // 1コピー
//     fn parse_feature<G: DeserializeGeometry, P: DeserializeProperties>(self) -> (G, P) {
//         let geo = match self.0 {
//             // 2コピー
//             Shape::Polyline(x) => Geometry::MultiLineString(x.into()),
//             _ => todo!(),
//         };
//         let g = G::deserialize_geometry(geo);
//         let p = todo!();
//         (g, p)
//     }
// }

// ゼロコピー だけど2dオンリー
// struct Geometry0(geo_types::Geometry);

// ゼロコピー 理想形
struct Geometry0<T>(T);
impl<T: DeserializeGeometry> ReadableShape for Geometry0<T> {
    fn read_from<R: std::io::Read>(
        source: &mut R,
        record_size: i32,
    ) -> Result<Self, shapefile::Error> {
        // let geom = T::deserialize_geometry(fmt);
        let geom = todo!();
        Ok(Self(geom))
    }
}

// Shape の時点で1コピー
// geo_types は unstable なところが不安
// struct Shape0(Shape);
// impl MultiLineStringTrait for Shape0 {
//     type T = f64;

//     type LineStringType<'a>
//     where
//         Self: 'a;

//     fn dim(&self) -> geo_traits::Dimensions {
//         todo!()
//     }

//     fn num_line_strings(&self) -> usize {
//         todo!()
//     }

//     unsafe fn line_string_unchecked(&self, i: usize) -> Self::LineStringType<'_> {
//         todo!()
//     }
// }

// 要するに、形式への参照 → geo_types::Geometry および、 geo_typesの型への参照 → 形式 の双方向変換ができればよい
// しかし geo_types は 2d である
