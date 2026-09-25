#![cfg_attr(all(doc, not(doctest)), feature(doc_cfg))]

//! Geoserde is an adapter between Rust data structures and geospatial file formats.
//!
//! It bridges [serde]-based property serialization with geometry handling,
//! letting you read and write geospatial features as plain Rust structs.
//!
//! * **[`de`]** — Deserialize geometries from GIS sources into Rust types.
//! * **[`ser`]** — Serialize struct properties into flat key-value tables.
//!
//! # Getting started
//!
//! ```sh
//! cargo add geoserde
//! ```
//!
//! To read and write [FlatGeobuf](https://flatgeobuf.org/) files, use the
//! [geoserde-fgb](https://docs.rs/geoserde-fgb) crate.
//!
//! # Cargo features
//!
//! * `geo` — [`DeserializeGeometry`] impls for [`geo_types`]. Enabled by default.

pub mod de;
pub mod ser;

pub use de::DeserializeGeometry;
pub use ser::SerializeProperties;
