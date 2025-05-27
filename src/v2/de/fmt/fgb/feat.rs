use flatgeobuf::{FgbFeature, Header};

use std::io::Cursor;

use crate::v2::de::{DeserializeGeometry, DeserializeProperties, ParseFeature};

use super::prop::PropertiesParser;

pub struct FeatureParser<'a> {
    header: Header<'a>,
    feature: &'a FgbFeature,
}

impl<'a> FeatureParser<'a> {
    pub fn new(header: Header<'a>, feature: &'a FgbFeature) -> Self {
        Self { header, feature }
    }
}

impl ParseFeature for FeatureParser<'_> {
    fn parse_feature<G: DeserializeGeometry, P: DeserializeProperties>(self) -> (G, P) {
        let g_fmt = self.feature.geometry_trait().unwrap().unwrap();
        let g = G::deserialize_geometry(g_fmt);

        let p_reader = Cursor::new(self.feature.fbs_feature().properties().unwrap().bytes());
        let p_fmt = &mut PropertiesParser::new(self.header, p_reader);
        let p = P::deserialize_properties(p_fmt);
        (g, p)
    }
}
