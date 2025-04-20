use crate::v2::de::{DeserializeGeometry, DeserializeProperties, ParseFeature};

impl ParseFeature for &'_ flatgeobuf::FgbFeature {
    fn parse_feature<G: DeserializeGeometry, P: DeserializeProperties>(self) -> (G, P) {
        let g = G::deserialize_geometry(self.geometry_trait().unwrap().unwrap());
        let p = P::deserialize_properties(&mut super::geozero::PropertiesAdapter::new(self));
        (g, p)
    }
}
