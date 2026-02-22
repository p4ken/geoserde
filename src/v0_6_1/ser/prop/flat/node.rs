use serde::{
    ser::{Error, Impossible, StdError},
    Serialize, Serializer,
};

pub enum TextLike {
    Empty,
    Bool(bool),
    Signed(i64),
    Unsigned(u64),
    Float(f64),
    Char(char),
    String(String),
    Ident(&'static str),
}

impl std::fmt::Display for TextLike {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextLike::Bool(v) => write!(f, "{v}"),
            TextLike::Signed(v) => write!(f, "{v}"),
            TextLike::Unsigned(v) => write!(f, "{v}"),
            TextLike::Float(v) => write!(f, "{v}"),
            TextLike::Char(v) => write!(f, "{v}"),
            TextLike::String(v) => write!(f, "{v}"),
            TextLike::Ident(v) => write!(f, "{v}"),
            TextLike::Empty => f.write_str(""),
        }
    }
}

pub struct TextLikeSerializer;

impl Serializer for TextLikeSerializer {
    type Ok = TextLike;
    type Error = TextError;

    type SerializeSeq = Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(TextLike::Bool(v))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v.into())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v.into())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v.into())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(TextLike::Signed(v))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(v.into())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(v.into())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(v.into())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(TextLike::Unsigned(v))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(v.into())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(TextLike::Float(v))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(TextLike::Char(v))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(TextLike::String(v.into()))
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(TextError::NotText)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(TextLike::Empty)
    }

    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<Self::Ok, Self::Error> {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(TextLike::Empty)
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(TextLike::Ident(name))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(TextLike::Ident(variant))
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        value.serialize(self)
    }
    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        value.serialize(self)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(TextError::NotText)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(TextError::NotText)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(TextError::NotText)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(TextError::NotText)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(TextError::NotText)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(TextError::NotText)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(TextError::NotText)
    }
}

#[derive(Debug)]
pub enum TextError {
    NotText,
    Serialize(serde::de::value::Error),
}

impl Error for TextError {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Self::Serialize(Error::custom(msg))
    }
}

impl StdError for TextError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Serialize(e) => Some(e),
            _ => None,
        }
    }
}

impl std::fmt::Display for TextError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextError::NotText => f.write_str("not a text"),
            TextError::Serialize(_) => f.write_str("serialize impl caused"),
        }
    }
}
