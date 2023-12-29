use serde::{
    ser::{
        SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
        SerializeTupleStruct, SerializeTupleVariant,
    },
    Serialize, Serializer,
};

use crate::{GeometrySerializer, GeometrySink, PropertySerializer, PropertySink, SerializeError};

pub trait FeatureSink: GeometrySink + PropertySink<Error = <Self as GeometrySink>::Error> {
    fn properties_start(&mut self) -> Result<(), <Self as GeometrySink>::Error>;
    fn properties_end(&mut self) -> Result<(), <Self as GeometrySink>::Error>;
    fn feature_start(&mut self, index: usize) -> Result<(), <Self as GeometrySink>::Error>;
    fn feature_end(&mut self, index: usize) -> Result<(), <Self as GeometrySink>::Error>;
}
#[cfg(feature = "geozero")]
impl<Z: geozero::FeatureProcessor> FeatureSink for Z {
    fn properties_start(&mut self) -> Result<(), <Self as GeometrySink>::Error> {
        self.properties_begin()
    }
    fn properties_end(&mut self) -> Result<(), <Self as GeometrySink>::Error> {
        self.properties_end()
    }
    fn feature_start(&mut self, index: usize) -> Result<(), <Self as GeometrySink>::Error> {
        self.feature_begin(index.try_into().unwrap())
    }
    fn feature_end(&mut self, index: usize) -> Result<(), <Self as GeometrySink>::Error> {
        self.feature_end(index.try_into().unwrap())
    }
}

/// Serializer for features.
///
/// A feature is a struct that have a geometry and may also have properties.
///
/// # Geometry detection
///
/// Geometry is the feature's field that can be serialized with [`GeometrySerializer`].
///
/// By default, only the first geometry
///
/// # Examples
///
/// ```
#[doc = include_str!("../../examples/serialize.rs")]
/// ```
pub struct FeatureSerializer<'a, S: FeatureSink> {
    sink: &'a mut S,
    geom_key: &'static str,
    feat_index: usize,
    remaining_field: usize,
    has_geom: bool,
    prop_index: usize,
}

impl<'a, S: FeatureSink> FeatureSerializer<'a, S> {
    /// Create a new `FeatureSerializer` with a [`FeatureSink`].
    pub fn new(sink: &'a mut S) -> Self {
        Self {
            sink,
            geom_key: "geometry", // FIXME: make empty by default
            feat_index: 0,
            remaining_field: 0,
            has_geom: false,
            prop_index: 0,
        }
    }
    pub fn geometry_key(&mut self, key: &'static str) -> &Self {
        self.geom_key = key;
        self
    }
    pub fn count(&self) -> usize {
        self.feat_index
    }
}

impl<'a, S: FeatureSink> Serializer for &mut FeatureSerializer<'a, S> {
    type Ok = ();
    type Error = SerializeError<<S as GeometrySink>::Error>;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::InvalidFeatureStructure)
    }

    fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::InvalidFeatureStructure)
    }

    fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::InvalidFeatureStructure)
    }

    fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::InvalidFeatureStructure)
    }

    fn serialize_i64(self, _: i64) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::InvalidFeatureStructure)
    }

    fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::InvalidFeatureStructure)
    }

    fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::InvalidFeatureStructure)
    }

    fn serialize_u32(self, _: u32) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::InvalidFeatureStructure)
    }

    fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::InvalidFeatureStructure)
    }

    fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::InvalidFeatureStructure)
    }

    fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::InvalidFeatureStructure)
    }

    fn serialize_char(self, _: char) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::InvalidFeatureStructure)
    }

    fn serialize_str(self, _: &str) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::InvalidFeatureStructure)
    }

    fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::InvalidFeatureStructure)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        // flatten it
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::InvalidFeatureStructure)
    }

    fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::InvalidFeatureStructure)
    }

    fn serialize_unit_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::InvalidFeatureStructure)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        // flatten it
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        // flatten it
        value.serialize(self)
    }

    fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(self)
    }

    fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        // field key is required
        Err(SerializeError::InvalidFeatureStructure)
    }

    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        // field key is required
        Err(SerializeError::InvalidFeatureStructure)
    }

    fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        // field key is required
        Err(SerializeError::InvalidFeatureStructure)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.remaining_field = len;
        self.sink
            .feature_start(self.feat_index)
            .map_err(SerializeError::SinkCaused)?;
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.serialize_struct(name, len)
    }
}

impl<'a, S: FeatureSink> SerializeSeq for &mut FeatureSerializer<'a, S> {
    type Ok = ();
    type Error = SerializeError<<S as GeometrySink>::Error>;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, S: FeatureSink> SerializeTuple for &mut FeatureSerializer<'a, S> {
    type Ok = ();
    type Error = SerializeError<<S as GeometrySink>::Error>;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        SerializeSeq::serialize_element(&mut *self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        SerializeSeq::end(&mut *self)
    }
}
impl<'a, S: FeatureSink> SerializeTupleStruct for &mut FeatureSerializer<'a, S> {
    type Ok = ();
    type Error = SerializeError<<S as GeometrySink>::Error>;

    fn serialize_field<T: ?Sized>(&mut self, _: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}

impl<'a, S: FeatureSink> SerializeTupleVariant for &mut FeatureSerializer<'a, S> {
    type Ok = ();
    type Error = SerializeError<<S as GeometrySink>::Error>;

    fn serialize_field<T: ?Sized>(&mut self, _: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}
impl<'a, S: FeatureSink> SerializeMap for &mut FeatureSerializer<'a, S> {
    type Ok = ();
    type Error = SerializeError<<S as GeometrySink>::Error>;

    fn serialize_key<T: ?Sized>(&mut self, _: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn serialize_value<T: ?Sized>(&mut self, _: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}

impl<'a, S: FeatureSink> SerializeStruct for &mut FeatureSerializer<'a, S> {
    type Ok = ();
    type Error = SerializeError<<S as GeometrySink>::Error>;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.remaining_field = self
            .remaining_field
            .checked_sub(1)
            .ok_or(SerializeError::InvalidState)?;

        if !self.has_geom {
            // try to serialize as a geometry
            let mut geom = GeometrySerializer::new(self.sink);
            match value.serialize(&mut geom) {
                // found the first geometry field
                Ok(()) => {
                    self.has_geom = true;
                    /* May have to cache geometry type */
                    return Ok(());
                }

                // the field must be a geometry
                Err(e) if key == self.geom_key => return Err(e),

                // ignore the error
                // FIXME: some junk may have been written
                Err(_) => (),
            }
        }

        if self.prop_index == 0 {
            self.sink
                .properties_start()
                .map_err(SerializeError::SinkCaused)?;
        }

        // serialize as a property
        let mut prop = PropertySerializer::new(self.prop_index, key, self.sink);
        value.serialize(&mut prop)?;
        self.prop_index = prop.index();

        if self.remaining_field == (if self.has_geom { 0 } else { 1 }) {
            self.sink
                .properties_end()
                .map_err(SerializeError::SinkCaused)?;
        }

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        if !self.has_geom {
            return Err(SerializeError::NoGeometryField);
        }

        self.sink
            .feature_end(self.feat_index)
            .map_err(SerializeError::SinkCaused)?;
        self.feat_index += 1;
        self.remaining_field = 0;
        self.has_geom = false;
        self.prop_index = 0;
        Ok(())
    }
}

impl<'a, S: FeatureSink> SerializeStructVariant for &mut FeatureSerializer<'a, S> {
    type Ok = ();
    type Error = SerializeError<<S as GeometrySink>::Error>;

    fn serialize_field<T: ?Sized>(&mut self, _: &'static str, _: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}
