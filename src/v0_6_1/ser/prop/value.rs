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
    Str(&'a str),
    Bytes(&'a [u8]),
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
        Self::Str(value)
    }
}

impl<'a> From<&'a [u8]> for FieldValue<'a> {
    fn from(value: &'a [u8]) -> Self {
        Self::Bytes(value)
    }
}
