use std::marker::PhantomData;

use serde::{
    ser::{Impossible, SerializeMap, SerializeSeq, SerializeStruct, SerializeTuple},
    Serialize, Serializer,
};

use crate::{FeatureSink, GeometrySerializer, PropertySerializer, SerializeError};

struct KeySerializer<S> {
    value: String,
    _phantom: PhantomData<S>,
}
impl<S> KeySerializer<S> {
    pub fn new() -> Self {
        KeySerializer {
            value: String::new(),
            _phantom: PhantomData::default(),
        }
    }
    pub fn to_string(self) -> String {
        self.value
    }
}
impl<'a, S: FeatureSink> Serializer for &'a mut KeySerializer<S> {
    type Ok = ();
    type Error = SerializeError<S::FeatErr>;

    type SerializeSeq = Impossible<Self::Ok, Self::Error>;

    type SerializeTuple = Impossible<Self::Ok, Self::Error>;

    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;

    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;

    type SerializeMap = Impossible<Self::Ok, Self::Error>;

    type SerializeStruct = Impossible<Self::Ok, Self::Error>;

    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.value += &v.to_string();
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.value += &v.to_string();
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.value += &v.to_string();
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.value += &v.to_string();
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.value += &v.to_string();
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.value += &v.to_string();
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.value += &v.to_string();
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.value += &v.to_string();
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.value += &v.to_string();
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.value += &v.to_string();
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.value += &v.to_string();
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.value += &v.to_string();
        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.value += v;
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.value += "None";
        Ok(())
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.value += " - ";
        Ok(())
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.value += name;
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        todo!()
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        todo!()
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        todo!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        todo!()
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        todo!()
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        todo!()
    }
}
/// Serialize geographic features to GIS formats.
///
/// # Geometry detection
///
/// The first geo-types field in the feature struct is serialized as a geometry.
///
/// Any other fields are properties.
///
/// Every features must have a geometry and may also have some properties.
///
/// Geometry and properties are serialized with [`GeometrySerializer`] and [`PropertySerializer`].
pub struct FeatureSerializer<'a, S> {
    sink: &'a mut S,
    // geom_key: Option<&'static str>,
    feat_index: usize,
    remaining_field: usize,
    has_geom: bool,
    prop_index: usize,
}

impl<'a, S: FeatureSink> FeatureSerializer<'a, S> {
    /// Create a new `FeatureSerializer` with a [`FeatureSink`].
    ///
    /// # Examples
    ///
    /// ```
    /// let mut sink = geozero::ProcessorSink;
    /// let mut ser = geoserde::FeatureSerializer::new(&mut sink);
    /// ```
    pub fn new(sink: &'a mut S) -> Self {
        Self {
            sink,
            // geom_key: None,
            feat_index: 0,
            remaining_field: 0,
            has_geom: false,
            prop_index: 0,
        }
    }
}

impl<S> FeatureSerializer<'_, S> {
    // fn geometry_key(mut self, key: &'static str) -> Self {
    //     self.geom_key = Some(key);
    //     self
    // }

    /// The number of features written to the sink.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut sink = geozero::ProcessorSink;
    /// let mut ser = geoserde::FeatureSerializer::new(&mut sink);
    /// println!("{} features", ser.len());  // => 0 features
    /// ```
    pub fn len(&self) -> usize {
        self.feat_index
    }
}

impl<'a, S: FeatureSink> Serializer for &mut FeatureSerializer<'a, S> {
    type Ok = ();
    type Error = SerializeError<S::FeatErr>;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

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

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        // field key is required
        self.sink
            .feature_start(self.feat_index)
            .map_err(SerializeError::SinkCaused)?;
        self.sink
            .properties_start()
            .map_err(SerializeError::SinkCaused)?;
        Ok(self)
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
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        // every features must be a same structures
        Err(SerializeError::InvalidFeatureStructure)
    }
}

impl<'a, S: FeatureSink> SerializeSeq for &mut FeatureSerializer<'a, S> {
    type Ok = ();
    type Error = SerializeError<S::FeatErr>;

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
    type Error = SerializeError<S::FeatErr>;

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

impl<'a, S: FeatureSink> SerializeMap for &mut FeatureSerializer<'a, S> {
    type Ok = ();
    type Error = SerializeError<S::FeatErr>;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }
    fn serialize_entry<K, V>(&mut self, key: &K, value: &V) -> Result<(), Self::Error>
    where
        K: ?Sized + Serialize,
        V: ?Sized + Serialize,
    {
        if !self.has_geom {
            // try to serialize as a geometry
            let mut geom = GeometrySerializer::new(self.sink);
            match value.serialize(&mut geom) {
                // found the first geometry field
                Ok(()) => {
                    self.has_geom = true;
                    return Ok(());
                }

                // failed but some junk was written
                Err(e) if geom.is_sink_used() => return Err(e),

                // it's just a property
                Err(_) => (),
            }
        }

        // serialize as a property
        let mut key_serializer = KeySerializer::<S>::new();
        key.serialize(&mut key_serializer)?;
        let key_str = key_serializer.to_string();
        let mut prop = PropertySerializer::new(self.prop_index, &key_str, self.sink);
        value.serialize(&mut prop)?;
        self.prop_index = prop.len();

        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!();
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        if !self.has_geom {
            return Err(SerializeError::NoGeometryField);
        }

        self.sink
            .properties_end()
            .map_err(SerializeError::SinkCaused)?;
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
impl<'a, S: FeatureSink> SerializeStruct for &mut FeatureSerializer<'a, S> {
    type Ok = ();
    type Error = SerializeError<S::FeatErr>;

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
                    return Ok(());
                }

                // failed but some junk was written
                Err(e) if geom.is_sink_used() => return Err(e),

                // it's just a property
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
        self.prop_index = prop.len();

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
