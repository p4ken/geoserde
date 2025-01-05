pub trait SerializeFeature {
    fn serialize_geometry<S, C>(&self, ser: S) -> Result<(), S::Error>
    where
        S: GeometrySerializer<C>,
        C: geo_types::CoordNum;

    fn serialize_properties<'a, S>(
        &self,
        ser: &'a mut S,
    ) -> Result<<&'a mut S as serde::Serializer>::Ok, <&'a mut S as serde::Serializer>::Error>
    where
        &'a mut S: serde::Serializer;
}
impl SerializeFeature for geo_types::Point {
    fn serialize_geometry<S, C>(&self, _ser: S) -> Result<(), S::Error>
    where
        S: GeometrySerializer<C>,
        C: geo_types::CoordNum,
    {
        todo!()
    }

    fn serialize_properties<'a, S>(
        &self,
        _ser: &'a mut S,
    ) -> Result<<&'a mut S as serde::Serializer>::Ok, <&'a mut S as serde::Serializer>::Error>
    where
        &'a mut S: serde::Serializer,
    {
        todo!()
    }
}

// pub trait DeserializeFeature {
//     fn deserialize_geometry(&self, : ) -> Result<>
// }

pub trait GeometrySerializer<C: geo_types::CoordNum> {
    type Error: std::error::Error;
    fn serialize_point(self, point: &geo_types::Point) -> Result<(), Self::Error>;
    fn serialize_line_string(self, line_string: &[&C]) -> Result<(), Self::Error>;
    fn serialize_multi_line_string(self, multi_line_string: &[&[&C]]) -> Result<(), Self::Error>;
}

pub struct GeometrySink {}
impl<C: geo_types::CoordNum> GeometrySerializer<C> for GeometrySink {
    type Error = std::io::Error; // tmp
    fn serialize_point(self, _point: &geo_types::Point) -> Result<(), Self::Error> {
        todo!()
    }
    fn serialize_line_string(self, _line_string: &[&C]) -> Result<(), Self::Error> {
        todo!()
    }
    fn serialize_multi_line_string(self, _multi_line_string: &[&[&C]]) -> Result<(), Self::Error> {
        todo!()
    }
}
