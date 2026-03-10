use crate::v0_6_2::de::DeserializeGeometry;

pub struct FeatureDeserializer<R> {
    fgb_reader: flatgeobuf::FgbReader<R>,
}

impl<R> FeatureDeserializer<R> {
    pub fn new(fgb_reader: flatgeobuf::FgbReader<R>) -> Self {
        Self { fgb_reader }
    }

    pub fn deserialize_feature<G: DeserializeGeometry, P: serde::de::DeserializeOwned>(
        &mut self,
    ) -> Result<(G, P), flatgeobuf::Error> {
        todo!()
    }
}
