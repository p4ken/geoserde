//! Property serialization into flat key-value tables.
//!
//! This module flattens nested Rust structs and maps into a single layer of
//! key-value pairs suitable for attribute tables in GIS formats. Nested keys
//! are joined with a configurable separator (e.g. `"parent.child"`).
//!
//! See [`TableSerializer`] for the main entry point.

mod prop;

pub use prop::{
    FieldValue, FlatProperties, FlattenOption, SerializeProperties, TableError, TableSerializer,
    flatten_keys,
};

/// An error originating from the data source during property serialization.
#[derive(Debug)]
pub struct SourceError(String);

impl From<String> for SourceError {
    fn from(string: String) -> Self {
        Self(string)
    }
}

impl std::fmt::Display for SourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for SourceError {}

impl serde::ser::Error for SourceError {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Self(msg.to_string())
    }
}
