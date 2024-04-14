#![cfg_attr(all(doc, not(doctest)), feature(doc_auto_cfg))]

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
//! * `geozero` - Implement geoserde sink for geozero processors. Enabled by default.
//!
//! # Examples
//!
//! ```
#![doc = include_str!("../examples/serialize.rs")]
//! ```

mod ser;
mod de;

pub use crate::ser::*;
