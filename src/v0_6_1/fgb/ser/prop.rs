use std::{borrow::Cow, fmt::Display};

use flatgeobuf::FgbWriter;

use crate::v0_6_1::{
    fgb::ser::value,
    ser::{
        prop::{value::FlatValue, SerializeProperties},
        SourceError,
    },
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

    pub fn into_inner(self) -> FgbWriter<'a> {
        self.writer
    }
}

impl<'a> SerializeProperties for PropertiesSerializer<'a> {
    type Ok = FgbWriter<'a>;
    type Error = PropertiesError;

    fn serialize_property<'b>(
        &mut self,
        key: Cow<'static, str>,
        value: FlatValue<'b>,
    ) -> Result<(), Self::Error> {
        let index_of_key = self.known_key.iter().position(|k| k == &key);
        flatgeobuf::geozero::PropertyProcessor::property(
            &mut self.writer,
            index_of_key.unwrap_or_else(|| self.known_key.len()),
            key.as_ref(),
            &value::from_flat_value(value),
        )
        .unwrap();

        if index_of_key.is_none() {
            self.known_key.push(key);
        };
        Ok(())
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

impl std::error::Error for PropertiesError {}

impl serde::ser::Error for PropertiesError {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::Source(msg.to_string().into())
    }
}
