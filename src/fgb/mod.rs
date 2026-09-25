#![cfg(feature = "fgb")]

//! [FlatGeobuf](https://flatgeobuf.org/) serialization and deserialization.
//!
//! Use [`FeatureDeserializer`] to read features from a FlatGeobuf file, and
//! [`FeatureSerializer`] to write them.

mod de;
mod error;
mod ser;

pub use de::{FeatureDeserializer, Features};
pub use error::{Error, FeatureError, PropertyError};
pub use ser::FeatureSerializer;

pub use flatgeobuf;
