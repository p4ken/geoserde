use std::fmt::Display;

use flatgeobuf::FgbWriter;
use serde::{
    ser::{Error, StdError},
    Serializer,
};

pub struct PropertySerializer<'a> {
    writer: FgbWriter<'a>,
    key: String,
}

impl<'a> PropertySerializer<'a> {
    pub fn new(writer: FgbWriter<'a>) -> Self {
        Self {
            writer,
            key: String::new(),
        }
    }

    pub fn into_inner(self) -> FgbWriter<'a> {
        self.writer
    }
}

// TODO: Find index by key. If not found, increment index.

#[derive(Debug)]
pub struct PropertiesError;

impl Display for PropertiesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Properties serialization error")
    }
}

impl StdError for PropertiesError {}

impl Error for PropertiesError {
    fn custom<T>(_msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self
    }
}
