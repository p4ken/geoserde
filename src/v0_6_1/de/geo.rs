use serde::Deserialize;

use crate::v0_6_1::{de::FromLineStringSeq, DeserializeGeometry};

impl From<crate::v0_6_1::Point> for geo_types::Coord {
    fn from(p: crate::v0_6_1::Point) -> Self {
        [p.x, p.y].into()
    }
}

impl DeserializeGeometry for geo_types::Coord {
    fn deserialize_geometry<'a, D: serde::Deserializer<'a>>(de: D) -> Result<Self, D::Error> {
        let p = crate::v0_6_1::Point::deserialize(de)?;
        Ok(geo_types::Coord::from(p))
    }
}

impl DeserializeGeometry for geo_types::Point {
    fn deserialize_geometry<'a, D: serde::Deserializer<'a>>(de: D) -> Result<Self, D::Error> {
        let p = crate::v0_6_1::Point::deserialize(de)?;
        Ok(geo_types::Point::new(p.x, p.y))
    }
}

impl DeserializeGeometry for geo_types::LineString {
    fn deserialize_geometry<'a, D: serde::Deserializer<'a>>(de: D) -> Result<Self, D::Error> {
        let vec = crate::v0_6_1::LineString::deserialize(de)?.0;
        Ok(geo_types::LineString(vec))
    }
}

impl DeserializeGeometry for geo_types::Line {
    fn deserialize_geometry<'a, D: serde::Deserializer<'a>>(de: D) -> Result<Self, D::Error> {
        let [start, end] =
            crate::v0_6_1::LineString::<[crate::v0_6_1::Point; _]>::deserialize(de)?.0;
        Ok(geo_types::Line::new(start, end))
    }
}

impl DeserializeGeometry for geo_types::Rect {
    fn deserialize_geometry<'a, D: serde::Deserializer<'a>>(de: D) -> Result<Self, D::Error> {
        let [c1, _, c2, _] =
            crate::v0_6_1::LineString::<[crate::v0_6_1::Point; _]>::deserialize(de)?.0;
        Ok(geo_types::Rect::new(c1, c2))
    }
}

impl DeserializeGeometry for geo_types::Triangle {
    fn deserialize_geometry<'a, D: serde::Deserializer<'a>>(de: D) -> Result<Self, D::Error> {
        let [v1, v2, v3] = crate::v0_6_1::LineString::<[crate::v0_6_1::Point; _]>::deserialize(de)?
            .0
            .map(Into::into);
        Ok(geo_types::Triangle::new(v1, v2, v3))
    }
}

impl DeserializeGeometry for geo_types::Polygon {
    fn deserialize_geometry<'a, D: serde::Deserializer<'a>>(de: D) -> Result<Self, D::Error> {
        let polygon = crate::v0_6_1::Polygon::deserialize(de)?.0;
        Ok(polygon)
    }
}

impl FromLineStringSeq for geo_types::Polygon {
    fn from_linestring_seq<'de, A: serde::de::SeqAccess<'de>>(
        mut seq: A,
    ) -> Result<Self, A::Error> {
        // Deserialize the first LineString
        let exterior = match seq.next_element::<crate::v0_6_1::LineString<_>>()? {
            Some(ls) => geo_types::LineString(ls.0),
            None => return Ok(geo_types::Polygon::empty()),
        };

        // Optimize heap allocation
        let mut interiors = Vec::with_capacity(seq.size_hint().unwrap_or(0).saturating_sub(1));

        // Deserialize LineStrings
        while let Some(ls) = seq.next_element::<crate::v0_6_1::LineString<_>>()? {
            interiors.push(geo_types::LineString(ls.0));
        }

        Ok(geo_types::Polygon::new(exterior, interiors))
    }
}

// impl DeserializeGeometry for geo_types::Geometry {
//     fn deserialize_geometry<'a, D: serde::Deserializer<'a>>(de: D) -> Result<Self, D::Error> {
//         let [c1, _, c2, _] = super::LineString::<[super::Point; _]>::deserialize(de)?.0;
//         Ok(geo_types::Rect::new(c1, c2))
//     }
// }
