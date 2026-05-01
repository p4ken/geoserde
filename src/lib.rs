#![cfg_attr(all(doc, not(doctest)), feature(doc_cfg))]

//! Geoserde is an adapter between geographic feature structs and GIS file formats.
//!
//! # Getting started
//!
//! ```sh
//! cargo add geoserde
//! ```
//!
//! # Cargo features
//!
//! * `fgb` - FlatGeobuf serialization / deserialization. Enabled by default.
//! * `geo` - `DeserializeGeometry` impls for `geo_types`. Enabled by default.

pub mod de;
#[cfg(feature = "fgb")]
pub mod fgb;
#[cfg(feature = "geo")]
mod geo;
pub mod ser;

pub use de::DeserializeGeometry;
