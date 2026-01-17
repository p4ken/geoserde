use serde::Deserialize;

use crate::v0_6_1::DeserializeGeometry;

impl From<super::Point> for geo_types::Coord {
    fn from(p: super::Point) -> Self {
        [p.x, p.y].into()
    }
}

impl DeserializeGeometry for geo_types::Coord {
    fn deserialize_geometry<'a, D: serde::Deserializer<'a>>(de: D) -> Result<Self, D::Error> {
        super::Point::deserialize(de).map(Into::into)
    }
}

impl DeserializeGeometry for geo_types::Point {
    fn deserialize_geometry<'a, D: serde::Deserializer<'a>>(de: D) -> Result<Self, D::Error> {
        geo_types::Coord::deserialize_geometry(de).map(Into::into)
    }
}

impl DeserializeGeometry for geo_types::LineString {
    fn deserialize_geometry<'a, D: serde::Deserializer<'a>>(de: D) -> Result<Self, D::Error> {
        let vec = super::LineString::deserialize(de)?.0;
        Ok(geo_types::LineString(vec))
    }
}

impl DeserializeGeometry for geo_types::Line {
    fn deserialize_geometry<'a, D: serde::Deserializer<'a>>(de: D) -> Result<Self, D::Error> {
        let [start, end] = super::LineString::<[super::Point; _]>::deserialize(de)?.0;
        Ok(geo_types::Line::new(start, end))
    }
}

impl DeserializeGeometry for geo_types::Rect {
    fn deserialize_geometry<'a, D: serde::Deserializer<'a>>(de: D) -> Result<Self, D::Error> {
        let [c1, _, c2, _, _] = super::LineString::<[super::Point; _]>::deserialize(de)?.0;
        Ok(geo_types::Rect::new(c1, c2))
    }
}

impl DeserializeGeometry for geo_types::Triangle {
    fn deserialize_geometry<'a, D: serde::Deserializer<'a>>(de: D) -> Result<Self, D::Error> {
        let [v1, v2, v3, _] = super::LineString::<[super::Point; _]>::deserialize(de)?
            .0
            .map(Into::into);
        Ok(geo_types::Triangle::new(v1, v2, v3))
    }
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
