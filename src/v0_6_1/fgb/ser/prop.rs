use std::{borrow::Cow, fmt::Display};

use flatgeobuf::FgbWriter;

use crate::v0_6_1::ser::{
    prop::{FieldValue, SerializeProperties},
    SourceError,
};

pub struct PropertiesSerializer<'a> {
    writer: FgbWriter<'a>,
    known_key: Vec<Cow<'static, str>>,
}

impl<'a> PropertiesSerializer<'a> {
    pub fn new(writer: FgbWriter<'a>) -> Self {
        Self {
            writer,
            known_key: Vec::new(),
        }
    }
}

impl<'a> SerializeProperties for PropertiesSerializer<'a> {
    type Ok = FgbWriter<'a>;
    type Error = PropertiesError;

    fn serialize_property(
        &mut self,
        key: Cow<'static, str>,
        value: FieldValue<'_>,
    ) -> Result<(), Self::Error> {
        let index_of_key = self.known_key.iter().position(|k| k == &key);
        flatgeobuf::geozero::PropertyProcessor::property(
            &mut self.writer,
            index_of_key.unwrap_or_else(|| self.known_key.len()),
            key.as_ref(),
            &_to_column_value(value),
        )?;

        if index_of_key.is_none() {
            self.known_key.push(key);
        };
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.writer)
    }
}

fn _to_column_value(source: FieldValue<'_>) -> flatgeobuf::geozero::ColumnValue<'_> {
    match source {
        FieldValue::Bool(v) => flatgeobuf::geozero::ColumnValue::Bool(v),
        FieldValue::I8(v) => flatgeobuf::geozero::ColumnValue::Byte(v),
        FieldValue::I16(v) => flatgeobuf::geozero::ColumnValue::Short(v),
        FieldValue::I32(v) => flatgeobuf::geozero::ColumnValue::Int(v),
        FieldValue::I64(v) => flatgeobuf::geozero::ColumnValue::Long(v),
        FieldValue::U8(v) => flatgeobuf::geozero::ColumnValue::UByte(v),
        FieldValue::U16(v) => flatgeobuf::geozero::ColumnValue::UShort(v),
        FieldValue::U32(v) => flatgeobuf::geozero::ColumnValue::UInt(v),
        FieldValue::U64(v) => flatgeobuf::geozero::ColumnValue::ULong(v),
        FieldValue::F32(v) => flatgeobuf::geozero::ColumnValue::Float(v),
        FieldValue::F64(v) => flatgeobuf::geozero::ColumnValue::Double(v),
        FieldValue::Str(s) => flatgeobuf::geozero::ColumnValue::String(s),
        FieldValue::Bytes(b) => flatgeobuf::geozero::ColumnValue::Binary(b),
    }
}

#[derive(Debug)]
pub enum PropertiesError {
    Fgb(flatgeobuf::Error),
    Geozero(flatgeobuf::geozero::error::GeozeroError),
    Source(SourceError),
}

impl From<flatgeobuf::Error> for PropertiesError {
    fn from(error: flatgeobuf::Error) -> Self {
        Self::Fgb(error)
    }
}

impl From<flatgeobuf::geozero::error::GeozeroError> for PropertiesError {
    fn from(error: flatgeobuf::geozero::error::GeozeroError) -> Self {
        Self::Geozero(error)
    }
}

impl Display for PropertiesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Properties serialization error")
    }
}

impl std::error::Error for PropertiesError {}

impl serde::ser::Error for PropertiesError {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::Source(msg.to_string().into())
    }
}
