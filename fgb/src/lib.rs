#![doc = include_str!("../README.md")]

mod de;
mod error;
mod ser;

pub use de::{FeatureDeserializer, Features};
pub use error::{Error, FeatureError, PropertyError};
pub use ser::FeatureSerializer;

pub use flatgeobuf;
