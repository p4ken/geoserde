use std::borrow::Cow;

use serde::Serialize;

use super::{FieldValue, SerializeProperties, TableError, TableSerializer};
use crate::ser::SourceError;

/// Flattens a serializable value and collects only its property keys,
/// discarding the values.
///
/// This is useful for inspecting the schema of a struct before writing
/// features.
///
/// # Errors
///
/// Returns [`TableError`] if the source value is not a struct or map.
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
