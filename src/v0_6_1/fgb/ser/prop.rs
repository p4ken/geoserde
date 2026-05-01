use std::{borrow::Cow, fmt::Display};

use flatgeobuf::FgbWriter;

use crate::v0_6_1::ser::{FieldValue, SerializeProperties, SourceError};

// CLEANUP-v0.6: superseded by v0_6_2::fgb::ser::FeatureSerializer; the helpers `to_column_type` / `to_column_value` below are still used by v0_6_2 and should stay
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

    // TODO: Delegate to FeatureSerializer
    pub fn mut_inner(&mut self) -> &mut FgbWriter<'a> {
        &mut self.writer
    }

    pub fn into_inner(self) -> FgbWriter<'a> {
        self.writer
    }
}

impl<'a> SerializeProperties for &mut PropertiesSerializer<'a> {
    type Ok = ();
    type Error = PropertiesError;

    fn serialize_property(
        &mut self,
        key: Cow<'static, str>,
        value: FieldValue<'_>,
    ) -> Result<(), Self::Error> {
        let index_of_key = self.known_key.iter().position(|k| k == &key);
        let index_to_write = index_of_key.unwrap_or_else(|| self.known_key.len());
        flatgeobuf::geozero::PropertyProcessor::property(
            &mut self.writer,
            index_to_write,
            key.as_ref(),
            &to_column_value(&value),
        )?;

        if index_of_key.is_none() {
            self.known_key.push(key);
        };
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

pub fn to_column_type(source: &FieldValue<'_>) -> flatgeobuf::ColumnType {
    match source {
        FieldValue::Bool(_) => flatgeobuf::ColumnType::Bool,
        FieldValue::I8(_) => flatgeobuf::ColumnType::Byte,
        FieldValue::I16(_) => flatgeobuf::ColumnType::Short,
        FieldValue::I32(_) => flatgeobuf::ColumnType::Int,
        FieldValue::I64(_) => flatgeobuf::ColumnType::Long,
        FieldValue::U8(_) => flatgeobuf::ColumnType::UByte,
        FieldValue::U16(_) => flatgeobuf::ColumnType::UShort,
        FieldValue::U32(_) => flatgeobuf::ColumnType::UInt,
        FieldValue::U64(_) => flatgeobuf::ColumnType::ULong,
        FieldValue::F32(_) => flatgeobuf::ColumnType::Float,
        FieldValue::F64(_) => flatgeobuf::ColumnType::Double,
        FieldValue::Str(_) | FieldValue::BoxedStr(_) => flatgeobuf::ColumnType::String,
        FieldValue::Bytes(_) | FieldValue::BoxedBytes(_) => flatgeobuf::ColumnType::Binary,
    }
}

pub fn to_column_value<'a>(source: &'a FieldValue<'_>) -> flatgeobuf::geozero::ColumnValue<'a> {
    match source {
        FieldValue::Bool(v) => flatgeobuf::geozero::ColumnValue::Bool(*v),
        FieldValue::I8(v) => flatgeobuf::geozero::ColumnValue::Byte(*v),
        FieldValue::I16(v) => flatgeobuf::geozero::ColumnValue::Short(*v),
        FieldValue::I32(v) => flatgeobuf::geozero::ColumnValue::Int(*v),
        FieldValue::I64(v) => flatgeobuf::geozero::ColumnValue::Long(*v),
        FieldValue::U8(v) => flatgeobuf::geozero::ColumnValue::UByte(*v),
        FieldValue::U16(v) => flatgeobuf::geozero::ColumnValue::UShort(*v),
        FieldValue::U32(v) => flatgeobuf::geozero::ColumnValue::UInt(*v),
        FieldValue::U64(v) => flatgeobuf::geozero::ColumnValue::ULong(*v),
        FieldValue::F32(v) => flatgeobuf::geozero::ColumnValue::Float(*v),
        FieldValue::F64(v) => flatgeobuf::geozero::ColumnValue::Double(*v),
        FieldValue::Str(s) => flatgeobuf::geozero::ColumnValue::String(s),
        FieldValue::BoxedStr(s) => flatgeobuf::geozero::ColumnValue::String(s),
        FieldValue::Bytes(b) => flatgeobuf::geozero::ColumnValue::Binary(b),
        FieldValue::BoxedBytes(b) => flatgeobuf::geozero::ColumnValue::Binary(b),
    }
}

#[derive(Debug)]
pub enum PropertiesError {
    Fgb(flatgeobuf::Error),
    Geozero(flatgeobuf::geozero::error::GeozeroError),
    Source(SourceError),
}

impl From<flatgeobuf::Error> for PropertiesError {
    fn from(e: flatgeobuf::Error) -> Self {
        Self::Fgb(e)
    }
}

impl From<flatgeobuf::geozero::error::GeozeroError> for PropertiesError {
    fn from(e: flatgeobuf::geozero::error::GeozeroError) -> Self {
        Self::Geozero(e)
    }
}

impl Display for PropertiesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PropertiesError::Fgb(_) => f.write_str("flatgeobuf writer failed"),
            PropertiesError::Geozero(_) => f.write_str("flatgeobuf::geozero error"),
            PropertiesError::Source(_) => f.write_str("serialize impl for source type failed"),
        }
    }
}

impl std::error::Error for PropertiesError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            PropertiesError::Fgb(e) => Some(e),
            PropertiesError::Geozero(e) => Some(e),
            PropertiesError::Source(e) => Some(e),
        }
    }
}

impl serde::ser::Error for PropertiesError {
    fn custom<T: Display>(msg: T) -> Self {
        Self::Source(msg.to_string().into())
    }
}
