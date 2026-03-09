use std::borrow::Cow;

use flatgeobuf::FgbWriter;

pub struct FeatureSerializer<'a> {
    writer: FgbWriter<'a>,
    known_key: Vec<Cow<'static, str>>,
}

impl<'a> FeatureSerializer<'a> {
    pub fn new(writer: FgbWriter<'a>) -> Self {
        Self {
            writer,
            known_key: Vec::new(),
        }
    }

    pub fn serialize_feature(
        &mut self,
        geometry: impl geo_traits::GeometryTrait,
        properties: impl serde::Serialize,
    ) -> Result<(), flatgeobuf::Error> {
        // TODO
        flatgeobuf::geozero::FeatureProcessor::feature_end(&mut self.writer, 0).unwrap(); // TODO
        Ok(())
    }
}
