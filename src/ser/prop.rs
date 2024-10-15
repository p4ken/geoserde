use serde::{
    ser::{
        SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
        SerializeTupleStruct, SerializeTupleVariant,
    },
    Serialize, Serializer,
};

use crate::{PropertySink, SerializeError};

/// Serialize properties to GIS formats.
///
/// # Panics
///
/// Multi-value types like `tuple`, `Vec`, `HashMap` are not supported yet, so panic.
pub struct PropertySerializer<'a, S> {
    index: usize,
    key: &'a str,
    sink: &'a mut S,
}

impl<'a, S: PropertySink> PropertySerializer<'a, S> {
    /// Create a new `PropertySerializer` with a [`PropertySink`].
    ///
    /// # Examples
    ///
    /// ```
    /// let mut sink = geozero::ProcessorSink;
    /// let mut ser = geoserde::PropertySerializer::new(0, "spot_name", &mut sink);
    /// ```
    pub fn new(index: usize, key: &'a str, sink: &'a mut S) -> Self {
        Self { index, key, sink }
    }
}

impl<S> PropertySerializer<'_, S> {
    /// The number of properties written to the sink.
    ///
    /// # Examples
    ///
    /// ```
    /// # use serde::ser::Serialize;
    ///
    /// let mut sink = geozero::ProcessorSink;
    /// let mut ser = geoserde::PropertySerializer::new(0, "name", &mut sink);
    /// assert_eq!(ser.len(), 0);
    ///
    /// "Sydney".serialize(&mut ser);
    /// assert_eq!(ser.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.index
    }
}

impl<S: PropertySink> Serializer for &mut PropertySerializer<'_, S> {
    type Ok = ();
    type Error = SerializeError<S::Err>;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.sink
            .bool(self.index, self.key, v)
            .map_err(SerializeError::SinkCaused)?;
        self.index += 1;
        Ok(())
    }
    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.sink
            .i8(self.index, self.key, v)
            .map_err(SerializeError::SinkCaused)?;
        self.index += 1;
        Ok(())
    }
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.sink
            .i16(self.index, self.key, v)
            .map_err(SerializeError::SinkCaused)?;
        self.index += 1;
        Ok(())
    }
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.sink
            .i32(self.index, self.key, v)
            .map_err(SerializeError::SinkCaused)?;
        self.index += 1;
        Ok(())
    }
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.sink
            .i64(self.index, self.key, v)
            .map_err(SerializeError::SinkCaused)?;
        self.index += 1;
        Ok(())
    }
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.sink
            .u8(self.index, self.key, v)
            .map_err(SerializeError::SinkCaused)?;
        self.index += 1;
        Ok(())
    }
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.sink
            .u16(self.index, self.key, v)
            .map_err(SerializeError::SinkCaused)?;
        self.index += 1;
        Ok(())
    }
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.sink
            .u32(self.index, self.key, v)
            .map_err(SerializeError::SinkCaused)?;
        self.index += 1;
        Ok(())
    }
    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.sink
            .u64(self.index, self.key, v)
            .map_err(SerializeError::SinkCaused)?;
        self.index += 1;
        Ok(())
    }
    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.sink
            .f32(self.index, self.key, v)
            .map_err(SerializeError::SinkCaused)?;
        self.index += 1;
        Ok(())
    }
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.sink
            .f64(self.index, self.key, v)
            .map_err(SerializeError::SinkCaused)?;
        self.index += 1;
        Ok(())
    }
    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&v.to_string())
    }
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.sink
            .str(self.index, self.key, v)
            .map_err(SerializeError::SinkCaused)?;
        self.index += 1;
        Ok(())
    }
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.sink
            .bytes(self.index, self.key, v)
            .map_err(SerializeError::SinkCaused)?;
        self.index += 1;
        Ok(())
    }
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.index += 1;
        Ok(())
    }
    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        // flatten it
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        // skip it
        Ok(())
    }

    fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> {
        // skip it
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.sink
            .str(self.index, name, variant)
            .map_err(SerializeError::SinkCaused)?;
        Ok(())
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        // flatten it
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        // FIXME: use value
        self.sink
            .str(self.index, name, variant)
            .map_err(SerializeError::SinkCaused)?;
        self.index += 1;
        Ok(())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(self)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(self)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(self)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(self)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(self)
    }
}

impl<S: PropertySink> SerializeSeq for &mut PropertySerializer<'_, S> {
    type Ok = ();
    type Error = SerializeError<S::Err>;

    fn serialize_element<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        // FIXME: to csv string
        Err(SerializeError::UnsupportedPropertyStructure { actual: "seq" })
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<S: PropertySink> SerializeTuple for &mut PropertySerializer<'_, S> {
    type Ok = ();
    type Error = SerializeError<S::Err>;
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

impl<S: PropertySink> SerializeTupleStruct for &mut PropertySerializer<'_, S> {
    type Ok = ();
    type Error = SerializeError<S::Err>;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        // FIXME: to key_1, key_2 ...
        Err(SerializeError::UnsupportedPropertyStructure { actual: "tuple" })
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<S: PropertySink> SerializeTupleVariant for &mut PropertySerializer<'_, S> {
    type Ok = ();
    type Error = SerializeError<S::Err>;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        SerializeTuple::serialize_element(&mut *self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        SerializeTuple::end(&mut *self)
    }
}

impl<S: PropertySink> SerializeMap for &mut PropertySerializer<'_, S> {
    type Ok = ();
    type Error = SerializeError<S::Err>;

    fn serialize_key<T: ?Sized>(&mut self, _key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        Err(SerializeError::UnsupportedPropertyStructure { actual: "map" })
    }

    fn serialize_value<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        Err(SerializeError::UnsupportedPropertyStructure { actual: "map" })
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<S: PropertySink> SerializeStruct for &mut PropertySerializer<'_, S> {
    type Ok = ();
    type Error = SerializeError<S::Err>;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.key = key;
        value.serialize(&mut **self)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<S: PropertySink> SerializeStructVariant for &mut PropertySerializer<'_, S> {
    type Ok = ();
    type Error = SerializeError<S::Err>;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        SerializeStruct::serialize_field(&mut *self, key, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        SerializeStruct::end(self)
    }
}
