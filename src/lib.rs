#![cfg_attr(all(doc, not(doctest)), feature(doc_cfg))]

//! Geoserde is an adapter between Rust data structures and geospatial file formats.
//!
//! It bridges [serde]-based property serialization with geometry handling,
//! letting you read and write geospatial features as plain Rust structs.
//!
//! * **[`de`]** — Deserialize geometries from GIS sources into Rust types.
//! * **[`ser`]** — Serialize struct properties into flat key-value tables.
//! * **[`fgb`]** — Read and write [FlatGeobuf](https://flatgeobuf.org/) files.
//!
//! # Getting started
//!
//! ```sh
//! cargo add geoserde
//! ```
//!
//! # Cargo features
//!
//! * `fgb` — FlatGeobuf serialization / deserialization. Enabled by default.
//! * `geo` — [`DeserializeGeometry`] impls for [`geo_types`]. Enabled by default.

pub mod de;
pub mod fgb;
pub mod ser;

pub use de::DeserializeGeometry;
pub use ser::SerializeProperties;
