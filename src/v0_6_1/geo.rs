use serde::Deserialize;

use crate::v0_6_1::{DeserializeGeometry, Point};

impl DeserializeGeometry for geo_types::Point {
    fn deserialize<'a, D: serde::Deserializer<'a>>(de: D) -> Result<Self, D::Error> {
        let p = Point::deserialize(de)?;
        Ok(geo_types::Point::new(p.x, p.y))
    }
}

impl DeserializeGeometry for geo_types::LineString {
    fn deserialize<'a, D: serde::Deserializer<'a>>(de: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = geo_types::LineString;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence of Point")
            }
            fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
            where
                S: serde::de::SeqAccess<'de>,
            {
                let mut vec = Vec::with_capacity(seq.size_hint().unwrap_or(0));
                while let Some(c) = seq.next_element()?.map(parse_coord) {
                    vec.push(c);
                }
                Ok(vec.into())
            }
        }

        de.deserialize_newtype_struct("geoserde::LineString", Visitor)
    }
}

impl DeserializeGeometry for geo_types::Line {
    fn deserialize<'a, D: serde::Deserializer<'a>>(de: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = geo_types::Line;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence of Point")
            }
            fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
            where
                S: serde::de::SeqAccess<'de>,
            {
                let mut next = || match seq.next_element()? {
                    Some(p) => Ok(parse_coord(p)),
                    None => Err(serde::de::Error::custom("expected 2 Points")),
                };

                Ok(geo_types::Line::new(next()?, next()?))
            }
        }

        de.deserialize_newtype_struct("geoserde::LineString", Visitor)
    }
}
impl DeserializeGeometry for geo_types::Polygon {
    fn deserialize<'a, D: serde::Deserializer<'a>>(de: D) -> Result<Self, D::Error> {
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

fn parse_coord(p: Point) -> geo_types::Coord {
    [p.x, p.y].into()
}
