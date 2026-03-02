use std::borrow::Cow;

use serde::{
    ser::{
        Error, Impossible, SerializeMap, SerializeSeq, SerializeStruct, SerializeTuple,
        SerializeTupleStruct,
    },
    Serialize, Serializer,
};

use crate::v0_6_1::ser::prop::{
    node::{StringLike, Stringifier, StringifyError},
    value::FlatValue,
    FlattenError,
};

pub trait SerializeProperties {
    type Ok;
    type Error: Error;

    fn serialize_property<'a>(
        &mut self,
        key: Cow<'static, str>,
        value: FlatValue<'a>,
    ) -> Result<(), Self::Error>;

    fn end(self) -> Result<Self::Ok, Self::Error>;
}

impl<M: SerializeMap> SerializeProperties for M {
    type Ok = M::Ok;
    type Error = M::Error;

    fn serialize_property<'a>(
        &mut self,
        key: Cow<'static, str>,
        value: FlatValue<'a>,
    ) -> Result<(), Self::Error> {
        self.serialize_entry(&key, &value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.end()
    }
}

pub struct Child<P> {
    sink: P,
    index: usize,
    value_seq: Vec<StringLike>,
    key_stack: Vec<Cow<'static, str>>,
}

impl<P> Child<P> {
    pub fn new(sink: P) -> Self {
        Self {
            sink,
            index: 0,
            value_seq: Vec::new(),
            key_stack: Vec::new(),
        }
    }

    pub fn into_table(self) -> P {
        self.sink
    }

    fn build_key(&self) -> Cow<'static, str> {
        match self.key_stack.as_slice() {
            [Cow::Borrowed(single)] => Cow::Borrowed(*single),
            [multi @ ..] => Cow::Owned(multi.join(".")),
        }
    }
}

impl<P: SerializeProperties> Child<P> {
    fn _serialize_property<'a>(
        &mut self,
        value: impl Into<FlatValue<'a>>,
    ) -> Result<(), FlattenError<P::Error>> {
        let key = self.build_key();
        let value = value.into();
        self.sink
            .serialize_property(key, value)
            .map_err(FlattenError::Sink)
    }
}

impl<P: SerializeProperties<Error: 'static>> SerializeSeq for &mut Child<P> {
    type Ok = ();
    type Error = FlattenError<P::Error>;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        match value.serialize(Stringifier) {
            Ok(text) => {
                self.value_seq.push(text);
                return Ok(());
            }
            Err(StringifyError::Empty) => {
                self.value_seq.push(StringLike::Empty);
                return Ok(());
            }
            // FIXME: Previous elements are not serialized if error
            Err(_) => self.value_seq.clear(),
        }

        let parent_index = self.index;

        let parent_key = self.key_stack.pop();
        let key = Cow::Owned(format!(
            "{}[{}]",
            parent_key.as_ref().unwrap_or(&Cow::default()),
            parent_index
        ));
        self.key_stack.push(key);
        value.serialize(&mut **self)?;
        self.key_stack.pop();
        if let Some(p) = parent_key {
            self.key_stack.push(p);
        }

        self.index = parent_index + 1;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        if !self.value_seq.is_empty() {
            let value = self
                .value_seq
                .drain(..)
                .map(|text| text.to_string())
                .collect::<Vec<_>>()
                .join(",");
            self._serialize_property(FlatValue::Str(&value))?;
        }
        Ok(())
    }
}

impl<P: SerializeProperties<Error: 'static>> SerializeTuple for &mut Child<P> {
    type Ok = ();
    type Error = FlattenError<P::Error>;

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

impl<P: SerializeProperties<Error: 'static>> SerializeTupleStruct for &mut Child<P> {
    type Ok = ();
    type Error = FlattenError<P::Error>;

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

impl<P: SerializeProperties<Error: 'static>> SerializeStruct for &mut Child<P> {
    type Ok = ();
    type Error = FlattenError<P::Error>;

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

impl<'a, P: SerializeProperties<Error: 'static>> SerializeMap for &mut Child<P> {
    type Ok = ();
    type Error = FlattenError<P::Error>;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
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

impl<'a, P: SerializeProperties<Error: 'static>> Serializer for &'a mut Child<P> {
    type Ok = ();
    type Error = FlattenError<P::Error>;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self._serialize_property(v)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self._serialize_property(v)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self._serialize_property(v)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self._serialize_property(v)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self._serialize_property(v)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self._serialize_property(v)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self._serialize_property(v)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self._serialize_property(v)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self._serialize_property(v)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self._serialize_property(v)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self._serialize_property(v)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        let mut buf = [0; 4];
        let str = v.encode_utf8(&mut buf);
        self._serialize_property(&*str)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self._serialize_property(v)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self._serialize_property(v)
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
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self._serialize_property(variant)
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
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
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
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        todo!()
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
        todo!()
    }
}
