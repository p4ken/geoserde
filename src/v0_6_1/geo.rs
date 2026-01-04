use serde::Deserialize;

use crate::v0_6_1::DeserializeGeometry;

impl DeserializeGeometry for geo_types::Point {
    fn deserialize<'a, D: serde::Deserializer<'a>>(de: D) -> Result<Self, D::Error> {
        let p = super::Point::deserialize(de)?;
        Ok(geo_types::Point::new(p.x, p.y))
    }
}

impl DeserializeGeometry for geo_types::LineString {
    fn deserialize<'a, D: serde::Deserializer<'a>>(de: D) -> Result<Self, D::Error> {
        let vec = super::LineString::<Vec<_>>::deserialize(de)?.0;
        Ok(geo_types::LineString(vec))
    }
}

impl DeserializeGeometry for geo_types::Line {
    fn deserialize<'a, D: serde::Deserializer<'a>>(de: D) -> Result<Self, D::Error> {
        let [start, end] = super::LineString::<[_; _]>::deserialize(de)?
            .0
            .map(parse_coord);
        Ok(geo_types::Line::new(start, end))
    }
}

impl DeserializeGeometry for geo_types::Rect {
    fn deserialize<'a, D: serde::Deserializer<'a>>(de: D) -> Result<Self, D::Error> {
        let [c1, _, c2, _, _] = super::LineString::<[_; _]>::deserialize(de)?
            .0
            .map(parse_coord);
        Ok(geo_types::Rect::new(c1, c2))
    }
}

impl DeserializeGeometry for geo_types::Triangle {
    fn deserialize<'a, D: serde::Deserializer<'a>>(de: D) -> Result<Self, D::Error> {
        let [v1, v2, v3] = super::LineString::<[_; _]>::deserialize(de)?
            .0
            .map(parse_coord);
        Ok(geo_types::Triangle::new(v1, v2, v3))
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

fn parse_coord(p: super::Point) -> geo_types::Coord {
    [p.x, p.y].into()
}
