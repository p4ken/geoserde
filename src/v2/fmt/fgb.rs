use crate::v2::de::{DeserializeGeometry, DeserializeProperties, ParseFeature};

use super::geozero::GeozeroDeserializer;

impl ParseFeature for flatgeobuf::FgbFeature {
    fn parse_feature<G: DeserializeGeometry, P: DeserializeProperties>(self) -> (G, P) {
        let g = G::deserialize_geometry(self.geometry_trait().unwrap().unwrap());
        let p = P::deserialize_properties(GeozeroDeserializer::new(&self));
        (g, p)
    }
}
