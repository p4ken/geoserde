use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[cfg(feature = "flatgeobuf")]
pub mod fgb;

pub fn serialize<S: serde::Serializer>(
    geom: impl SerializeGeometry,
    ser: S,
) -> Result<S::Ok, S::Error> {
    geom.serialize(ser)
}

pub fn deserialize<'a, D: serde::Deserializer<'a>, G: DeserializeGeometry>(
    de: D,
) -> Result<G, D::Error> {
    G::deserialize_geometry(de)
}

pub trait SerializeGeometry: Serialize {}
impl SerializeGeometry for geo_types::Point {}
impl<T: SerializeGeometry> SerializeGeometry for &T {}

pub trait DeserializeGeometry: DeserializeOwned {
    fn deserialize_geometry<'a, D: serde::Deserializer<'a>>(de: D) -> Result<Self, D::Error> {
        todo!()
    }
    fn deserialize_geometry_2<'a, D: GeometryDeserializer<'a>>(de: D) -> Result<Self, D::Error> {
        todo!()
    }
}
impl DeserializeGeometry for geo_types::Point {
    fn deserialize_geometry<'a, D: serde::Deserializer<'a>>(de: D) -> Result<Self, D::Error> {
        Ok(Self::deserialize(de)?)
    }
}
impl DeserializeGeometry for geo_types::LineString {
    fn deserialize_geometry<'a, D: serde::Deserializer<'a>>(de: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Vec<geo_types::Coord>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence of Point")
            }
            fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
            where
                S: serde::de::SeqAccess<'de>,
            {
                let mut vec = Vec::with_capacity(seq.size_hint().unwrap_or(0));
                while let Some(point) = seq.next_element::<Point>()? {
                    vec.push(geo_types::Coord::from(point));
                }
                Ok(vec)
            }
        }

        // TODO: [Point]ではなくLineString([Point])に見せかけたい
        de.deserialize_seq(Visitor).map(geo_types::LineString)
    }
    // fn deserialize_geometry_2<'a, D: GeometryDeserializer<'a>>(de: D) -> Result<Self, D::Error> {
    //     struct Visitor;
    //     let visitor = Visitor;
    //     de.deserialize_line_string(visitor)
    // }
}
impl DeserializeGeometry for geo_types::Polygon {
    fn deserialize_geometry<'a, D: serde::Deserializer<'a>>(de: D) -> Result<Self, D::Error> {
        // TODO: ここを FgbFeature とか serde_json::Value とかに特化させる
        // DeserializeFlatgeobufGeometry とか DeserializeGeojsonGeometry になる
        // ただし #[geometry] だけでは不十分になる

        #[derive(Deserialize)]
        #[serde(rename = "geoserde::Polygon")]
        struct Polygon {
            // newtype LineString の vec でなければならない
            // この構造に限定するのは geo_types に依存しすぎである
            // geo_types はデファクトだが、Z座標がないのは致命的
            inner: Vec<geo_types::LineString>,
            outer: geo_types::LineString,
        }
        let sink = Polygon::deserialize(de)?;
        let dest = geo_types::Polygon::new(sink.outer, sink.inner);
        Ok(dest)
    }
}
// FIXME: data formats directly depend on geo_types structures.
// e.g. "Polygon" must have "exterior" field dispite it is private
// pub trait DeserializeGeometry: Sized {
//     fn deserialize_geometry(src: impl GeometryTrait<T = f64>) -> Self;
// }

pub trait GeometryDeserializer<'de> {
    type Error;
}

#[derive(Deserialize)]
#[serde(rename = "geoserde::Point")]
struct Point {
    #[serde(rename = "geoserde::X")]
    x: f64,
    #[serde(rename = "geoserde::Y")]
    y: f64,
    #[serde(rename = "geoserde::Z")]
    z: Option<f64>,
    #[serde(rename = "geoserde::M")]
    m: Option<f64>,
}
impl From<Point> for geo_types::Coord {
    fn from(src: Point) -> Self {
        Self { x: src.x, y: src.y }
    }
}

#[derive(Deserialize)]
#[serde(rename = "geoserde::LineString")]
struct LineString<T>(T);
