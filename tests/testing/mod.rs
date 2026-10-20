#![allow(dead_code)]

use std::{borrow::Cow, convert::Infallible};

use geoserde::ser;
use serde::Serialize;

pub type Entries = Vec<(String, ser::FieldValue<'static>)>;

/// Flattens `source` with [`ser::TableSerializer::new`].
pub fn flatten(source: impl Serialize) -> Result<Entries, ser::TableError<Infallible>> {
    let mut sink = Collector::default();
    source.serialize(ser::TableSerializer::new(&mut sink))?;
    Ok(sink.0)
}

/// Flattens `source` with [`ser::TableSerializer::with_options`].
pub fn flatten_with_options(
    source: impl Serialize,
    options: ser::FlattenOptions,
) -> Result<Entries, ser::TableError<Infallible>> {
    let mut sink = Collector::default();
    source.serialize(ser::TableSerializer::with_options(&mut sink, options))?;
    Ok(sink.0)
}

#[derive(Default)]
struct Collector(Entries);

impl ser::SerializeProperties for &mut Collector {
    type Ok = ();
    type Error = Infallible;

    fn serialize_property(
        &mut self,
        key: Cow<'static, str>,
        value: ser::FieldValue<'_>,
    ) -> Result<(), Self::Error> {
        self.0.push((key.into_owned(), value.into_owned()));
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}
