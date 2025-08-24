mod map;
mod prop;

use flatgeobuf::FgbFeature;
use prop::PropertiesAdapter;

use crate::v0_6::{DeserializeGeometry, DeserializeProperties, ParseFeature};

pub struct FeatureParser<'a> {
    feature: &'a FgbFeature,
}

impl<'a> FeatureParser<'a> {
    pub fn new(feature: &'a FgbFeature) -> Self {
        Self { feature }
    }
}

impl ParseFeature for FeatureParser<'_> {
    fn parse_feature<G: DeserializeGeometry, P: DeserializeProperties>(self) -> (G, P) {
        let g_fmt = self.feature.geometry_trait().unwrap().unwrap();
        let p_fmt = &mut PropertiesAdapter::new(self.feature);
        let g = G::deserialize_geometry(g_fmt);
        let p = P::deserialize_properties(p_fmt);
        (g, p)
    }
}
