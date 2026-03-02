use serde::{
    ser::{Error, Impossible, SerializeMap, SerializeStruct, StdError},
    Serialize, Serializer,
};

use crate::v0_6_1::ser::prop::{
    child::{Child, SerializeProperties},
    node::StringifyError,
};

pub struct FlattenSerializer<P> {
    child: Child<P>,
}

impl<P: SerializeProperties> FlattenSerializer<P> {
    pub fn new(sink: P) -> Self {
        Self {
            child: Child::new(sink),
        }
    }

    pub fn from_serializer<S: Serializer<SerializeMap = P>>(ser: S) -> Self {
        let sink = ser.serialize_map(None).unwrap();
        Self {
            child: Child::new(sink),
        }
    }

    pub fn into_inner(self) -> P {
        self.child.into_table()
    }
}

impl<P: SerializeProperties<Error: 'static>> Serializer for FlattenSerializer<P> {
    type Ok = P::Ok;
    type Error = FlattenError<P::Error>;

    type SerializeSeq = Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        Err(FlattenError::Root)
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        Err(FlattenError::Root)
    }

    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
        Err(FlattenError::Root)
    }

    fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
        Err(FlattenError::Root)
    }

    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
        Err(FlattenError::Root)
    }

    fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
        Err(FlattenError::Root)
    }

    fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
        Err(FlattenError::Root)
    }

    fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
        Err(FlattenError::Root)
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
        Err(FlattenError::Root)
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        Err(FlattenError::Root)
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        Err(FlattenError::Root)
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
        Err(FlattenError::Root)
    }

    fn serialize_str(self, _v: &str) -> Result<Self::Ok, Self::Error> {
        Err(FlattenError::Root)
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(FlattenError::Root)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(FlattenError::Root)
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(FlattenError::Root)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(FlattenError::Root)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(FlattenError::Root)
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
        Err(FlattenError::Root)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(FlattenError::Root)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(FlattenError::Root)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(FlattenError::Root)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(FlattenError::Root)
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
        Err(FlattenError::Root)
    }
}

impl<P: SerializeProperties<Error: 'static>> SerializeStruct for FlattenSerializer<P> {
    type Ok = P::Ok;
    type Error = FlattenError<P::Error>;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        (&mut self.child).serialize_field(key, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.child.into_table().end().map_err(FlattenError::Sink)
    }
}

impl<P: SerializeProperties<Error: 'static>> SerializeMap for FlattenSerializer<P> {
    type Ok = P::Ok;
    type Error = FlattenError<P::Error>;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        (&mut self.child).serialize_key(key)
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        (&mut self.child).serialize_value(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.child.into_table().end().map_err(FlattenError::Sink)
    }
}

#[derive(Debug)]
pub enum FlattenError<E> {
    Root,
    Key(StringifyError),
    Sink(E),
}

impl<E> From<StringifyError> for FlattenError<E> {
    fn from(e: StringifyError) -> Self {
        Self::Key(e)
    }
}

impl<E: std::fmt::Display> std::fmt::Display for FlattenError<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Root => f.write_str("data source must be a map or struct"),
            Self::Key(_) => f.write_str("map key must be a string"),
            Self::Sink(_) => f.write_str("downstream serializer caused"),
        }
    }
}

impl<E: StdError + 'static> StdError for FlattenError<E> {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Root => None,
            Self::Key(e) => Some(e),
            Self::Sink(e) => Some(e),
        }
    }
}

impl<E: Error + 'static> Error for FlattenError<E> {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Self::Sink(E::custom(msg))
    }
}
