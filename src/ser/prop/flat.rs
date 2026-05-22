use std::borrow::Cow;

use serde::Serialize;

use super::{FieldValue, SerializeProperties, TableError, TableSerializer};
use crate::ser::SourceError;

/// Flatten a serializable value and collect only its keys, discarding values.
pub fn flatten_keys(source: impl Serialize) -> Result<Vec<String>, TableError<SourceError>> {
    let mut keys = KeySink(Vec::new());
    let table_ser = TableSerializer::new(&mut keys);
    source.serialize(table_ser)?;
    Ok(keys.0.into_iter().map(|k| k.into_owned()).collect())
}

#[derive(Debug)]
struct KeySink(Vec<Cow<'static, str>>);

impl SerializeProperties for &mut KeySink {
    type Ok = ();
    type Error = SourceError;

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
    entries: Vec<(Cow<'static, str>, FieldValue<'static>)>,
}

impl FlatProperties {
    /// Flatten a serializable value into key-value pairs.
    pub fn flatten(source: impl Serialize) -> Result<Self, TableError<SourceError>> {
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
    pub fn get(&self, key: &str) -> Option<&FieldValue<'static>> {
        self.entries.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    /// Consume the flattened properties, returning the underlying entries.
    pub fn into_entries(self) -> Vec<(Cow<'static, str>, FieldValue<'static>)> {
        self.entries
    }
}

impl SerializeProperties for &mut FlatProperties {
    type Ok = ();
    type Error = SourceError;

    fn serialize_property(
        &mut self,
        key: Cow<'static, str>,
        value: FieldValue<'_>,
    ) -> Result<(), Self::Error> {
        self.entries.push((key, value.into_owned()));
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}
