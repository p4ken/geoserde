use serde::{
    ser::{Impossible, SerializeMap, SerializeSeq, SerializeStruct, SerializeTuple},
    Serialize, Serializer,
};

use crate::{FeatureSink, GeometrySerializer, PropertySerializer, SerializeError};

/// Serialize geographic features to GIS formats.
///
/// # Geometry detection
///
/// The first geo-types field in the feature struct is serialized as a geometry.
///
/// Rest of the fields are properties.
///
/// Every features must have a geometry and may also have some properties.
///
/// Geometry and properties are serialized with [`GeometrySerializer`] and [`PropertySerializer`].
///
/// # Examples
///
/// ```
#[doc = include_str!("../../examples/serialize.rs")]
/// ```
pub struct FeatureSerializer<'a, S> {
    sink: &'a mut S,
    // geom_key: Option<&'static str>,
    feat_index: usize,
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
            has_geom: false,
            prop_index: 0,
        }
    }

    fn start_feature(&mut self) -> Result<(), SerializeError<S::FeatErr>> {
        self.sink
            .feature_start(self.feat_index)
            .map_err(SerializeError::SinkCaused)
    }

    fn end_feature(&mut self) -> Result<(), SerializeError<S::FeatErr>> {
        if !self.has_geom {
            return Err(SerializeError::NoGeometryField);
        }

        if self.prop_index > 0 {
            self.sink
                .properties_end()
                .map_err(SerializeError::SinkCaused)?;
        }

        self.sink
            .feature_end(self.feat_index)
            .map_err(SerializeError::SinkCaused)?;
        self.feat_index += 1;
        self.has_geom = false;
        self.prop_index = 0;
        Ok(())
    }

    fn write_field(
        &mut self,
        key: &str,
        value: &(impl Serialize + ?Sized),
    ) -> Result<(), SerializeError<S::FeatErr>> {
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
        Ok(())
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
        Err(SerializeError::InvalidFeatureStructure("bool"))
    }

    fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::InvalidFeatureStructure("i8"))
    }

    fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::InvalidFeatureStructure("i16"))
    }

    fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::InvalidFeatureStructure("i32"))
    }

    fn serialize_i64(self, _: i64) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::InvalidFeatureStructure("i64"))
    }

    fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::InvalidFeatureStructure("u8"))
    }

    fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::InvalidFeatureStructure("u16"))
    }

    fn serialize_u32(self, _: u32) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::InvalidFeatureStructure("u32"))
    }

    fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::InvalidFeatureStructure("u64"))
    }

    fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::InvalidFeatureStructure("f32"))
    }

    fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::InvalidFeatureStructure("f64"))
    }

    fn serialize_char(self, _: char) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::InvalidFeatureStructure("char"))
    }

    fn serialize_str(self, _: &str) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::InvalidFeatureStructure("str"))
    }

    fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::InvalidFeatureStructure("bytes"))
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
        Err(SerializeError::InvalidFeatureStructure("unit"))
    }

    fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::InvalidFeatureStructure("unit struct"))
    }

    fn serialize_unit_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::InvalidFeatureStructure("variant"))
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
        Err(SerializeError::InvalidFeatureStructure("tuple struct"))
    }

    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        // field key is required
        Err(SerializeError::InvalidFeatureStructure("tuple variant"))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        self.start_feature()?;
        Ok(self)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.start_feature()?;
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
        Err(SerializeError::InvalidFeatureStructure("struct variant"))
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

    fn serialize_key<T: ?Sized>(&mut self, _: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        Err(SerializeError::InvalidFeatureStructure("separated k/v map"))
    }

    fn serialize_value<T: ?Sized>(&mut self, _: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        Err(SerializeError::InvalidState)
    }

    fn serialize_entry<K: ?Sized, V: ?Sized>(
        &mut self,
        key: &K,
        value: &V,
    ) -> Result<(), Self::Error>
    where
        K: Serialize,
        V: Serialize,
    {
        let key_str = key
            .serialize(serde_plain::Serializer)
            .map_err(|_| SerializeError::InvalidFeatureStructure("non str map"))?;
        self.write_field(&key_str, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.end_feature()
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
        self.write_field(key, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.end_feature()
    }
}

// derive_geoserde の方が良いかも？
// serialize が serde だけで可能か、それとも derive_geoserde が必要かどうか次第。
struct GeometryTrap<'a, S> {
    sink: &'a mut S,
}
