#![cfg_attr(all(doc, not(doctest)), feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]
//!
//! ## Examples
//!
//! Serialize features to geojson.
//!
//! ```
#![doc = include_str!("../examples/serialize.rs")]
//! ```

mod ser;

pub use ser::*;
