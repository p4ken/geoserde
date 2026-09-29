//! Reads and writes [FlatGeobuf](https://flatgeobuf.org/) features as your own
//! Rust data structures, built on [`geoserde`].
//!
//! Use [`FeatureDeserializer`] to read features from a FlatGeobuf file, and
//! [`FeatureSerializer`] to write them.
//!
//! # Cargo features
//!
//! * `geo` — Enables `geoserde/geo`. Enabled by default.

mod de;
mod error;
mod ser;

pub use de::{FeatureDeserializer, Features};
pub use error::{Error, FeatureError, PropertyError};
pub use ser::FeatureSerializer;

pub use flatgeobuf;
