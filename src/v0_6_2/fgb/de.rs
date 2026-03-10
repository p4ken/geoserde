pub struct FeatureDeserializer<R> {
    fgb_reader: flatgeobuf::FgbReader<R>,
}

impl<R> FeatureDeserializer<R> {
    pub fn new(fgb_reader: flatgeobuf::FgbReader<R>) -> Self {
        Self { fgb_reader }
    }

    pub fn deserialize_feature<G, P>(&mut self) -> Result<(G, P), flatgeobuf::Error> {
        todo!()
    }
}
