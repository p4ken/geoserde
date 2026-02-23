use serde::{
    ser::{Error, Impossible, StdError},
    Serialize, Serializer,
};

#[derive(Serialize)]
#[serde(untagged)]
pub enum StringLike {
    Empty,
    Bool(bool),
    Signed(i64),
    Unsigned(u64),
    Float(f64),
    Char(char),
    String(String),
    Ident(&'static str),
}

impl std::fmt::Display for StringLike {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str(""),
            Self::Bool(v) => write!(f, "{v}"),
            Self::Signed(v) => write!(f, "{v}"),
            Self::Unsigned(v) => write!(f, "{v}"),
            Self::Float(v) => write!(f, "{v}"),
            Self::Char(v) => write!(f, "{v}"),
            Self::String(v) => write!(f, "{v}"),
            Self::Ident(v) => write!(f, "{v}"),
        }
    }
}

pub struct Stringifier;

impl Serializer for Stringifier {
    type Ok = StringLike;
    type Error = StringifyError;

    type SerializeSeq = Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(StringLike::Bool(v))
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
        Ok(StringLike::Signed(v))
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
        Ok(StringLike::Unsigned(v))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(v.into())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(StringLike::Float(v))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(StringLike::Char(v))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        match v {
            "" => Err(StringifyError::Empty),
            _ => Ok(StringLike::String(v.into())),
        }
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(StringifyError::Nested)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(StringifyError::Empty)
    }

    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<Self::Ok, Self::Error> {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(StringifyError::Empty)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(StringifyError::Empty)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(StringLike::Ident(variant))
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
        _value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        Err(StringifyError::Nested)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(StringifyError::Nested)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(StringifyError::Nested)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(StringifyError::Nested)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(StringifyError::Nested)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(StringifyError::Nested)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(StringifyError::Nested)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(StringifyError::Nested)
    }
}

#[derive(Debug)]
pub enum StringifyError {
    Empty,
    Nested,
    Source(Box<dyn StdError>),
}

impl std::fmt::Display for StringifyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str("means empty"),
            Self::Nested => f.write_str("nested hierarchy"),
            Self::Source(_) => f.write_str("upstream serialize impl caused"),
        }
    }
}

impl StdError for StringifyError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Source(e) => Some(e.as_ref()),
            _ => None,
        }
    }
}

impl Error for StringifyError {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Self::Source(msg.to_string().into())
    }
}
