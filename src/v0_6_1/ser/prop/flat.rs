use std::borrow::Cow;

use serde::Serialize;

use super::{FieldValue, SerializeProperties, TableError, TableSerializer};

/// Owned representation of a field value.
pub enum PropertyValue {
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
    String(String),
    Bytes(Vec<u8>),
}

impl PropertyValue {
    pub fn as_field_value(&self) -> FieldValue<'_> {
        match self {
            Self::Bool(v) => FieldValue::Bool(*v),
            Self::I8(v) => FieldValue::I8(*v),
            Self::I16(v) => FieldValue::I16(*v),
            Self::I32(v) => FieldValue::I32(*v),
            Self::I64(v) => FieldValue::I64(*v),
            Self::U8(v) => FieldValue::U8(*v),
            Self::U16(v) => FieldValue::U16(*v),
            Self::U32(v) => FieldValue::U32(*v),
            Self::U64(v) => FieldValue::U64(*v),
            Self::F32(v) => FieldValue::F32(*v),
            Self::F64(v) => FieldValue::F64(*v),
            Self::String(s) => FieldValue::Str(s),
            Self::Bytes(b) => FieldValue::Bytes(b),
        }
    }
}

impl From<FieldValue<'_>> for PropertyValue {
    fn from(v: FieldValue<'_>) -> Self {
        match v {
            FieldValue::Bool(v) => Self::Bool(v),
            FieldValue::I8(v) => Self::I8(v),
            FieldValue::I16(v) => Self::I16(v),
            FieldValue::I32(v) => Self::I32(v),
            FieldValue::I64(v) => Self::I64(v),
            FieldValue::U8(v) => Self::U8(v),
            FieldValue::U16(v) => Self::U16(v),
            FieldValue::U32(v) => Self::U32(v),
            FieldValue::U64(v) => Self::U64(v),
            FieldValue::F32(v) => Self::F32(v),
            FieldValue::F64(v) => Self::F64(v),
            FieldValue::Str(s) => Self::String(s.to_owned()),
            FieldValue::Bytes(b) => Self::Bytes(b.to_owned()),
        }
    }
}

/// Flatten a serializable value and collect only its keys, discarding values.
pub fn flatten_keys(source: impl Serialize) -> Result<Vec<String>, TableError<Error>> {
    let mut keys = KeySink(Vec::new());
    let table_ser = TableSerializer::new(&mut keys);
    source.serialize(table_ser)?;
    Ok(keys.0.into_iter().map(|k| k.into_owned()).collect())
}

struct KeySink(Vec<Cow<'static, str>>);

impl SerializeProperties for &mut KeySink {
    type Ok = ();
    type Error = Error;

    fn serialize_property(
        &mut self,
        key: Cow<'static, str>,
        _value: FieldValue<'_>,
    ) -> Result<(), Self::Error> {
        self.0.push(key);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

/// Flattened key-value representation of a serializable object's properties.
///
/// Uses `TableSerializer` to recursively flatten nested structures into
/// dot-separated key-value pairs (e.g. `extra.z_col`).
pub struct FlatProperties {
    entries: Vec<(Cow<'static, str>, PropertyValue)>,
}

impl FlatProperties {
    /// Flatten a serializable value into key-value pairs.
    pub fn flatten(source: impl Serialize) -> Result<Self, TableError<Error>> {
        let mut collector = FlatProperties {
            entries: Vec::new(),
        };
        let table_ser = TableSerializer::new(&mut collector);
        source.serialize(table_ser)?;
        Ok(collector)
    }

    /// Returns an iterator over the property keys.
    pub fn keys(&self) -> impl Iterator<Item = &str> {
        self.entries.iter().map(|(k, _)| k.as_ref())
    }

    /// Look up a value by key name.
    pub fn get(&self, key: &str) -> Option<&PropertyValue> {
        self.entries.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }
}

impl SerializeProperties for &mut FlatProperties {
    type Ok = ();
    type Error = Error;

    fn serialize_property(
        &mut self,
        key: Cow<'static, str>,
        value: FieldValue<'_>,
    ) -> Result<(), Self::Error> {
        self.entries.push((key, PropertyValue::from(value)));
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct Error(String);

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for Error {}

impl serde::ser::Error for Error {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Self(msg.to_string())
    }
}
