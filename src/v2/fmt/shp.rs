use dbase::Record;
use geo_traits::{GeometryTrait, MultiLineStringTrait};
use geo_types::Geometry;
use serde::de::DeserializeOwned;
use shapefile::{ReadableShape, Shape};

use crate::v2::de::{DeserializeGeometry, ParseFeature};

// もはや deserialize もない
impl ParseFeature for (Shape, Record) {
    fn parse_feature<G: DeserializeGeometry, P: DeserializeOwned>(self) -> (G, P) {
        let geom = match self.0 {
            Shape::Polyline(x) => Geometry::MultiLineString(x.into()),
            _ => todo!(),
        };
        // 2コピー
        let geom = G::deserialize_geometry(geom);
        let prop = todo!();
        (geom, prop)
    }
}

// どうしてもゼロコピーな貴方へ
// struct Geometry0(geo_types::Geometry);
// impl ReadableShape for Geometry0 {
//     fn read_from<T: std::io::Read>(
//         source: &mut T,
//         record_size: i32,
//     ) -> Result<Self, shapefile::Error> {
//         let geom = todo!();
//         Ok(Self(geom))
//     }
// }

// 1コピー
// struct Shape0(Shape);
// impl MultiLineStringTrait for Shape0 {
//     type T;

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
