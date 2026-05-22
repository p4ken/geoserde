#![cfg(feature = "fgb")]

//! SerDe implementation for FlatGeobuf.
//!
//! It might be better to separate this module into other crate
//! like "geoserde-fgb" (or the official "flatgeobuf" crate).

pub mod de;
mod error;
pub mod ser;

pub use de::FeatureDeserializer;
pub use error::Error;
pub use ser::{FeatureSerializer, LayerSerializer};

pub use flatgeobuf;
