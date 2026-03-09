pub struct FeatureDeserializer {}

impl FeatureDeserializer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn deserialize_feature<G, P>(&mut self) -> Result<(G, P), flatgeobuf::Error> {
        todo!()
    }
}
