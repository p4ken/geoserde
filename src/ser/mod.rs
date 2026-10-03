//! Property serialization into flat key-value tables.
//!
//! This module flattens nested Rust structs and maps into a single layer of
//! key-value pairs suitable for attribute tables in GIS formats.
//!
//! See [`TableSerializer`] for the main entry point.
//!
//! # Flattening
//!
//! With the default [`FlattenOption::full`], the fields of the properties are
//! flattened as follows.
//!
//! | Field                                                          | Key                              | Value                                |
//! | -------------------------------------------------------------- | -------------------------------- | ------------------------------------ |
//! | `name: "Tokyo"`                                                | `name`                           | `Str("Tokyo")`                       |
//! | `address: Address { city: "Chiyoda" }`                         | `address.city`                   | `Str("Chiyoda")`                     |
//! | `#[serde(flatten)] address: Address { city: "Chiyoda" }`       | `city`                           | `Str("Chiyoda")`                     |
//! | `count: HashMap::from([("a", 1)])`                             | `count.a`                        | `I32(1)`                             |
//! | `lines: vec!["JY", "JC"]`                                      | `lines`                          | `Str("JY,JC")`                       |
//! | `exits: vec![Exit { name: "North" }, Exit { name: "South" }]`  | `exits[0].name`, `exits[1].name` | `Str("North")`, `Str("South")`       |
//! | `kind: Kind::Terminal`                                         | `kind`                           | `Str("Terminal")`                    |
//! | `shape: Shape::Circle(1.5)`                                    | `shape.Circle`                   | `F64(1.5)`                           |
//! | `capacity: Some(100u32)`                                       | `capacity`                       | `U32(100)`                           |
//! | `capacity: None`                                               | –                                | –                                    |
//!
//! - Values are [`FieldValue`]s, typed after the Rust type.
//! - An array of scalars, including a tuple, is joined into one string. An
//!   array holding a struct, a map or another array has a key per element.
//! - `None`, `()` and an empty array write no property.
//! - A struct variant has its fields under the variant name, as in
//!   `shape.Rect.width`.
//! - The separators `.`, `,`, `[` and `]` can be changed by [`FlattenOption`].
//! - A map key must be a string or another scalar, or [`TableError::Key`] is
//!   returned.
//! - The properties themselves must be a struct or a map, or
//!   [`TableError::Root`] is returned.
//!
//! [`flatten_keys`] shows the keys of a type.
//!
//! ```
//! #[derive(serde::Serialize)]
//! struct Station {
//!     name: &'static str,
//!     address: Address,
//! }
//!
//! #[derive(serde::Serialize)]
//! struct Address {
//!     city: &'static str,
//! }
//!
//! let station = Station { name: "Tokyo", address: Address { city: "Chiyoda" } };
//! assert_eq!(geoserde::ser::flatten_keys(&station)?, ["name", "address.city"]);
//! # Ok::<(), geoserde::ser::TableError<std::convert::Infallible>>(())
//! ```

mod elem;
mod field;
mod flat;
mod table;
mod value;

pub use field::SerializeProperties;
pub use flat::flatten_keys;
pub use table::{FlattenOption, KeyError, TableError, TableSerializer, ValueError};
pub use value::FieldValue;

/// An error originating from the data source during property serialization.
#[derive(Debug, Clone)]
pub(crate) struct SourceError(String);

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
