use std::borrow::Cow;

use serde::{
    ser::{
        Error, SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
        SerializeTupleStruct, SerializeTupleVariant,
    },
    Serialize, Serializer,
};

use crate::ser::prop::{
    elem::{StringLike, Stringifier, StringifyError},
    FieldValue, FlattenOption, TableError,
};

/// Trait for sinks that receive flattened key-value property pairs.
///
/// Implement this trait to consume the output of [`TableSerializer`](super::TableSerializer).
/// A blanket implementation is provided for any [`SerializeMap`].
pub trait SerializeProperties {
    /// The successful return type.
    type Ok;
    /// The error type.
    type Error: Error;

    /// Write a single key-value property pair.
    fn serialize_property(
        &mut self,
        key: Cow<'static, str>,
        value: FieldValue<'_>,
    ) -> Result<(), Self::Error>;

    /// Finish writing and return the result.
    fn end(self) -> Result<Self::Ok, Self::Error>;
}

impl<M: SerializeMap> SerializeProperties for M {
    type Ok = M::Ok;
    type Error = M::Error;

    fn serialize_property(
        &mut self,
        key: Cow<'static, str>,
        value: FieldValue<'_>,
    ) -> Result<(), Self::Error> {
        self.serialize_entry(&key, &value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.end()
    }
}

/// Serializer that recursively flattens nested structures into key-value pairs.
#[derive(Debug)]
pub struct FieldSerializer<P> {
    sink: P,
    option: FlattenOption,
    index: usize,
    key_stack: Vec<Cow<'static, str>>,
    value_seq: Vec<StringLike>,
}

impl<P: SerializeProperties> FieldSerializer<P> {
    pub fn new(sink: P, option: FlattenOption) -> Self {
        Self {
            sink,
            option,
            index: 0,
            key_stack: Vec::new(),
            value_seq: Vec::new(),
        }
    }

    pub fn into_inner(self) -> P {
        self.sink
    }

    fn build_key(&self) -> Cow<'static, str> {
        match self.key_stack.as_slice() {
            [Cow::Borrowed(single)] => Cow::Borrowed(*single),
            [multi @ ..] => Cow::Owned(multi.join(self.option.nested_attribute_separator())),
        }
    }

    fn serialize_value<'a>(
        &mut self,
        value: impl Into<FieldValue<'a>>,
    ) -> Result<(), TableError<P::Error>> {
        let key = self.build_key();
        let value = value.into();
        self.sink
            .serialize_property(key, value)
            .map_err(TableError::Sink)
    }
}

impl<P: SerializeProperties<Error: 'static>> FieldSerializer<P> {
    fn serialize_element_<T>(&mut self, value: &T) -> Result<(), TableError<P::Error>>
    where
        T: ?Sized + Serialize,
    {
        let parent_key = self.key_stack.pop(); // e.g. "parent"
        let parent_index = self.index;

        let key = Cow::Owned(format!(
            "{}{}{}{}",
            parent_key.as_ref().unwrap_or(&Cow::default()),
            self.option.nested_array_index_prefix(),
            parent_index,
            self.option.nested_array_index_suffix(),
        ));
        self.key_stack.push(key); // "parent[0]"
        value.serialize(&mut *self)?;
        self.key_stack.pop(); // "parent[0]"
        if let Some(p) = parent_key {
            self.key_stack.push(p); // "parent"
        }

        self.index = parent_index + 1;
        Ok(())
    }
}

impl<P: SerializeProperties<Error: 'static>> SerializeSeq for &mut FieldSerializer<P> {
    type Ok = ();
    type Error = TableError<P::Error>;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        if self.option.array_as_string() && self.index == 0 {
            match value.serialize(Stringifier) {
                Ok(text) => {
                    self.value_seq.push(text);
                    return Ok(());
                }
                Err(StringifyError::Empty) => {
                    self.value_seq.push(StringLike::Empty);
                    return Ok(());
                }
                Err(_) => {
                    // Fallback to "key[0]=value" style
                    let value_seq = std::mem::take(&mut self.value_seq);
                    for v in &value_seq {
                        self.serialize_element_(v)?;
                    }
                }
            }
        }

        self.serialize_element_(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        if !self.value_seq.is_empty() {
            // Flatten as "key=value0,value1" style
            let value = self
                .value_seq
                .drain(..)
                .map(|text| text.to_string())
                .collect::<Vec<_>>()
                .join(self.option.array_element_separator());
            self.serialize_value(value.as_str())?;
        }
        Ok(())
    }
}

impl<P: SerializeProperties<Error: 'static>> SerializeTuple for &mut FieldSerializer<P> {
    type Ok = ();
    type Error = TableError<P::Error>;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        SerializeSeq::end(self)
    }
}

impl<P: SerializeProperties<Error: 'static>> SerializeTupleStruct for &mut FieldSerializer<P> {
    type Ok = ();
    type Error = TableError<P::Error>;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        SerializeSeq::end(self)
    }
}

impl<P: SerializeProperties<Error: 'static>> SerializeStruct for &mut FieldSerializer<P> {
    type Ok = ();
    type Error = TableError<P::Error>;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.key_stack.push(Cow::Borrowed(key));
        value.serialize(&mut **self)?;
        self.key_stack.pop();
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, P: SerializeProperties<Error: 'static>> SerializeMap for &mut FieldSerializer<P> {
    type Ok = ();
    type Error = TableError<P::Error>;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        // WARNING: Some formats may accept empty keys
        let key = key.serialize(Stringifier)?;
        self.key_stack.push(key.into());
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)?;
        self.key_stack.pop();
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

pub struct VariantSerializer<'a, P> {
    inner: &'a mut FieldSerializer<P>,
}

impl<P: SerializeProperties<Error: 'static>> SerializeTupleVariant for VariantSerializer<'_, P> {
    type Ok = ();
    type Error = TableError<P::Error>;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        SerializeSeq::serialize_element(&mut self.inner, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let inner = self.inner;
        SerializeSeq::end(&mut *inner)?;
        inner.key_stack.pop();
        Ok(())
    }
}

impl<P: SerializeProperties<Error: 'static>> SerializeStructVariant for VariantSerializer<'_, P> {
    type Ok = ();
    type Error = TableError<P::Error>;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        SerializeStruct::serialize_field(&mut self.inner, key, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.inner.key_stack.pop();
        Ok(())
    }
}

impl<'a, P: SerializeProperties<Error: 'static>> Serializer for &'a mut FieldSerializer<P> {
    type Ok = ();
    type Error = TableError<P::Error>;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = VariantSerializer<'a, P>;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = VariantSerializer<'a, P>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.serialize_value(v)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_value(v)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.serialize_value(v)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.serialize_value(v)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.serialize_value(v)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.serialize_value(v)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.serialize_value(v)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.serialize_value(v)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.serialize_value(v)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.serialize_value(v)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.serialize_value(v)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        let mut buf = [0; 4];
        let str = v.encode_utf8(&mut buf);
        self.serialize_value(&*str)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.serialize_value(v)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.serialize_value(v)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_none()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_none()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_value(variant)
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.key_stack.push(Cow::Borrowed(variant));
        value.serialize(&mut *self)?;
        self.key_stack.pop();
        Ok(())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.index = 0;
        Ok(self)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.key_stack.push(Cow::Borrowed(variant));
        self.index = 0;
        Ok(VariantSerializer { inner: self })
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
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.key_stack.push(Cow::Borrowed(variant));
        Ok(VariantSerializer { inner: self })
    }
}
