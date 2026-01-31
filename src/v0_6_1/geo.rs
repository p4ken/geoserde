use serde::{de::VariantAccess, Deserialize};

use crate::v0_6_1::DeserializeGeometry;

impl From<super::Point> for geo_types::Coord {
    fn from(p: super::Point) -> Self {
        [p.x, p.y].into()
    }
}

impl DeserializeGeometry for geo_types::Coord {
    fn deserialize_geometry<'a, D: serde::Deserializer<'a>>(de: D) -> Result<Self, D::Error> {
        let p = super::Point::deserialize(de)?;
        Ok(geo_types::Coord::from(p))
    }
}

impl DeserializeGeometry for geo_types::Point {
    fn deserialize_geometry<'a, D: serde::Deserializer<'a>>(de: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = geo_types::Point;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "Geometry enum")
            }

            fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::EnumAccess<'de>,
            {
                // FIXME: String -> &str
                let (v, variant_access) = data.variant::<String>().unwrap();
                assert_eq!(v, "geoserde::Point");
                let p = variant_access.newtype_variant::<super::Point>().unwrap();
                Ok(geo_types::Point::new(p.x, p.y))
            }
        }
        de.deserialize_enum("", &["geoserde::Point"], Visitor)

        // let p = super::Point::deserialize(de)?;
        // Ok(geo_types::Point::new(p.x, p.y))
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
        let [c1, _, c2, _] = super::LineString::<[super::Point; _]>::deserialize(de)?.0;
        Ok(geo_types::Rect::new(c1, c2))
    }
}

impl DeserializeGeometry for geo_types::Triangle {
    fn deserialize_geometry<'a, D: serde::Deserializer<'a>>(de: D) -> Result<Self, D::Error> {
        let [v1, v2, v3] = super::LineString::<[super::Point; _]>::deserialize(de)?
            .0
            .map(Into::into);
        Ok(geo_types::Triangle::new(v1, v2, v3))
    }
}

impl DeserializeGeometry for geo_types::Polygon {
    fn deserialize_geometry<'a, D: serde::Deserializer<'a>>(de: D) -> Result<Self, D::Error> {
        let polygon = super::Polygon::deserialize(de)?.0;
        Ok(polygon)
    }
}
impl super::FromLineStringSeq for geo_types::Polygon {
    fn from_linestring_seq<'de, A: serde::de::SeqAccess<'de>>(
        mut seq: A,
    ) -> Result<Self, A::Error> {
        // Deserialize the first LineString
        let exterior = match seq.next_element::<super::LineString<_>>()? {
            Some(ls) => geo_types::LineString(ls.0),
            None => return Ok(geo_types::Polygon::empty()),
        };

        // Optimize heap allocation
        let mut interiors = Vec::with_capacity(seq.size_hint().unwrap_or(0).saturating_sub(1));

        // Deserialize LineStrings
        while let Some(ls) = seq.next_element::<super::LineString<_>>()? {
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
