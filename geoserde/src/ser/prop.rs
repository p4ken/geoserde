pub trait PropertySink {
    type Error: std::error::Error;
}
#[cfg(feature = "geozero")]
impl<G: geozero::GeomProcessor> PropertySink for G {
    type Error = geozero::error::GeozeroError;
}
pub struct PropertySerializer<'a, T> {
    sink: &'a T,
}
impl<'a, T: PropertySink> PropertySerializer<'a, T> {
    pub fn new(sink: &'a T) -> Self {
        Self { sink }
    }
}
