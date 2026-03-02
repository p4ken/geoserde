use std::{borrow::Cow, fmt::Display};

use flatgeobuf::FgbWriter;
use serde::{
    ser::{Error, Impossible, StdError},
    Serialize, Serializer,
};

use crate::v0_6_1::ser::{
    prop::{value::FlatValue, FlatProperties, SerializeProperties},
    SourceError,
};

pub struct PropertiesSerializer<'a> {
    writer: FgbWriter<'a>,
    // visited_key: Vec<
}

impl<'a> PropertiesSerializer<'a> {
    pub fn new(writer: FgbWriter<'a>) -> Self {
        Self { writer }
    }

    pub fn into_inner(self) -> FgbWriter<'a> {
        self.writer
    }
}

impl<'a> SerializeProperties for PropertiesSerializer<'a> {
    type Ok = FgbWriter<'a>;
    type Error = PropertiesError;

    fn serialize_property<'b>(
        &mut self,
        key: Cow<'b, str>,
        value: FlatValue<'b>,
    ) -> Result<(), Self::Error> {
        // self.writer.add_column(name, col_type, cfgfn);
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.writer)
    }
}

// TODO: Find index by key. If not found, increment index.

#[derive(Debug)]
pub enum PropertiesError {
    Fgb(flatgeobuf::Error),
    Source(SourceError),
}

impl From<flatgeobuf::Error> for PropertiesError {
    fn from(error: flatgeobuf::Error) -> Self {
        Self::Fgb(error)
    }
}

impl Display for PropertiesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Properties serialization error")
    }
}

impl StdError for PropertiesError {}

impl Error for PropertiesError {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::Source(msg.to_string().into())
    }
}
