#![cfg(feature = "fgb")]

//! [FlatGeobuf](https://flatgeobuf.org/) serialization and deserialization.
//!
//! Use [`FeatureDeserializer`] to read features from a FlatGeobuf file, and
//! [`FeatureSerializer`] to write them.

pub mod de;
mod error;
pub mod ser;

pub use de::{FeatureDeserializer, Features};
pub use error::Error;
pub use ser::FeatureSerializer;

pub use flatgeobuf;
