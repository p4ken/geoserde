use std::borrow::Cow;

use serde::Serialize;

#[derive(Serialize)]
#[serde(untagged)]
pub enum FieldValue<'a> {
    Bool(bool),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    F32(f32),
    F64(f64),
    Str(Cow<'a, str>),
    Bytes(Cow<'a, [u8]>),
}

impl FieldValue<'_> {
    pub fn into_owned(self) -> FieldValue<'static> {
        match self {
            Self::Bool(v) => FieldValue::Bool(v),
            Self::I8(v) => FieldValue::I8(v),
            Self::I16(v) => FieldValue::I16(v),
            Self::I32(v) => FieldValue::I32(v),
            Self::I64(v) => FieldValue::I64(v),
            Self::U8(v) => FieldValue::U8(v),
            Self::U16(v) => FieldValue::U16(v),
            Self::U32(v) => FieldValue::U32(v),
            Self::U64(v) => FieldValue::U64(v),
            Self::F32(v) => FieldValue::F32(v),
            Self::F64(v) => FieldValue::F64(v),
            Self::Str(s) => FieldValue::Str(Cow::Owned(s.into_owned())),
            Self::Bytes(b) => FieldValue::Bytes(Cow::Owned(b.into_owned())),
        }
    }
}

impl From<bool> for FieldValue<'static> {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<i8> for FieldValue<'static> {
    fn from(value: i8) -> Self {
        Self::I8(value)
    }
}

impl From<i16> for FieldValue<'static> {
    fn from(value: i16) -> Self {
        Self::I16(value)
    }
}

impl From<i32> for FieldValue<'static> {
    fn from(value: i32) -> Self {
        Self::I32(value)
    }
}

impl From<i64> for FieldValue<'static> {
    fn from(value: i64) -> Self {
        Self::I64(value)
    }
}

impl From<u8> for FieldValue<'static> {
    fn from(value: u8) -> Self {
        Self::U8(value)
    }
}

impl From<u16> for FieldValue<'static> {
    fn from(value: u16) -> Self {
        Self::U16(value)
    }
}

impl From<u32> for FieldValue<'static> {
    fn from(value: u32) -> Self {
        Self::U32(value)
    }
}

impl From<u64> for FieldValue<'static> {
    fn from(value: u64) -> Self {
        Self::U64(value)
    }
}

impl From<f32> for FieldValue<'static> {
    fn from(value: f32) -> Self {
        Self::F32(value)
    }
}

impl From<f64> for FieldValue<'static> {
    fn from(value: f64) -> Self {
        Self::F64(value)
    }
}

impl<'a> From<&'a str> for FieldValue<'a> {
    fn from(value: &'a str) -> Self {
        Self::Str(Cow::Borrowed(value))
    }
}

impl<'a> From<&'a [u8]> for FieldValue<'a> {
    fn from(value: &'a [u8]) -> Self {
        Self::Bytes(Cow::Borrowed(value))
    }
}

impl From<String> for FieldValue<'static> {
    fn from(value: String) -> Self {
        Self::Str(Cow::Owned(value))
    }
}

impl From<Vec<u8>> for FieldValue<'static> {
    fn from(value: Vec<u8>) -> Self {
        Self::Bytes(Cow::Owned(value))
    }
}
