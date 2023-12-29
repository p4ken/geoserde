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

pub use ser::err::SerializeError;
pub use ser::feat::{FeatureSerializer, FeatureSink};
pub use ser::geom::{GeometrySerializer, GeometrySink};
pub use ser::prop::{PropertySerializer, PropertySink};
