use std::convert::Infallible;

use geoserde::{FeatureSerializer, FeatureSink, GeometrySink, PropertySink, SerializeError};
use serde::Serialize;

fn serialize_feature(g: impl Serialize) -> (SinkMock, Option<SerializeError<Infallible>>) {
    let mut sink = SinkMock::default();
    let mut sut = FeatureSerializer::new(&mut sink);
    let err = g.serialize(&mut sut).err();
    (sink, err)
}

#[test]
fn no_geometry_test() {
    #[derive(Serialize)]
    struct MyFeature {
        value: f64,
    }
    let f = MyFeature { value: 0.0 };
    let (sink, err) = serialize_feature(f);
    assert_eq!(err, Some(SerializeError::NoGeometryField));
    assert!(!sink.wrote_geometry);
    assert!(sink.wrote_property);
}

#[derive(Debug, Default)]
struct SinkMock {
    wrote_geometry: bool,
    wrote_property: bool,
}
impl SinkMock {
    fn write_geometry(&mut self) {
        self.wrote_geometry = true;
    }
    fn write_property(&mut self) {
        self.wrote_property = true;
    }
}
impl GeometrySink for SinkMock {
    type Err = Infallible;
    fn coord(&mut self, _: usize, _: f64, _: f64) -> Result<(), Self::Err> {
        Ok(self.write_geometry())
    }
    fn point_start(&mut self, _: usize) -> Result<(), Self::Err> {
        Ok(self.write_geometry())
    }
    fn point_end(&mut self, _: usize) -> Result<(), Self::Err> {
        Ok(self.write_geometry())
    }
    fn linestring_start(&mut self, _: bool, _: usize, _: usize) -> Result<(), Self::Err> {
        Ok(self.write_geometry())
    }
    fn linestring_end(&mut self, _: bool, _: usize) -> Result<(), Self::Err> {
        Ok(self.write_geometry())
    }
    fn polygon_start(&mut self, _: bool, _: usize) -> Result<(), Self::Err> {
        Ok(self.write_geometry())
    }
    fn polygon_end(&mut self, _: bool, _: usize) -> Result<(), Self::Err> {
        Ok(self.write_geometry())
    }
    fn geometry_start(&mut self) -> Result<(), Self::Err> {
        Ok(self.write_geometry())
    }
    fn geometry_end(&mut self) -> Result<(), Self::Err> {
        Ok(self.write_geometry())
    }
}
impl PropertySink for SinkMock {
    type Err = Infallible;
    fn bool(&mut self, _: usize, _: &str, _: bool) -> Result<(), Self::Err> {
        Ok(self.write_property())
    }
    fn i8(&mut self, _: usize, _: &str, _: i8) -> Result<(), Self::Err> {
        Ok(self.write_property())
    }
    fn i16(&mut self, _: usize, _: &str, _: i16) -> Result<(), Self::Err> {
        Ok(self.write_property())
    }
    fn i32(&mut self, _: usize, _: &str, _: i32) -> Result<(), Self::Err> {
        Ok(self.write_property())
    }
    fn i64(&mut self, _: usize, _: &str, _: i64) -> Result<(), Self::Err> {
        Ok(self.write_property())
    }
    fn u8(&mut self, _: usize, _: &str, _: u8) -> Result<(), Self::Err> {
        Ok(self.write_property())
    }
    fn u16(&mut self, _: usize, _: &str, _: u16) -> Result<(), Self::Err> {
        Ok(self.write_property())
    }
    fn u32(&mut self, _: usize, _: &str, _: u32) -> Result<(), Self::Err> {
        Ok(self.write_property())
    }
    fn u64(&mut self, _: usize, _: &str, _: u64) -> Result<(), Self::Err> {
        Ok(self.write_property())
    }
    fn f32(&mut self, _: usize, _: &str, _: f32) -> Result<(), Self::Err> {
        Ok(self.write_property())
    }
    fn f64(&mut self, _: usize, _: &str, _: f64) -> Result<(), Self::Err> {
        Ok(self.write_property())
    }
    fn bytes(&mut self, _: usize, _: &str, _: &[u8]) -> Result<(), Self::Err> {
        Ok(self.write_property())
    }
    fn str(&mut self, _: usize, _: &str, _: &str) -> Result<(), Self::Err> {
        Ok(self.write_property())
    }
}
impl FeatureSink for SinkMock {
    type FeatErr = Infallible;
    fn properties_start(&mut self) -> Result<(), <Self as GeometrySink>::Err> {
        Ok(())
    }
    fn properties_end(&mut self) -> Result<(), <Self as GeometrySink>::Err> {
        Ok(())
    }
    fn feature_start(&mut self, _: usize) -> Result<(), <Self as GeometrySink>::Err> {
        Ok(())
    }
    fn feature_end(&mut self, _: usize) -> Result<(), <Self as GeometrySink>::Err> {
        Ok(())
    }
}
