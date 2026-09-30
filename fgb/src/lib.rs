#![doc = include_str!("../README.md")]

pub mod de;
pub mod ser;

pub use de::FeatureDeserializer;
pub use ser::FeatureSerializer;

pub use flatgeobuf;
