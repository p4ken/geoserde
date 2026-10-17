#![warn(unreachable_pub)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]
//!
//! # Traits for your data types
//!
//! | | Geometry | Properties |
//! |---|---|---|
//! | **Write** | [`geo_traits::GeometryTrait`] | [`serde::Serialize`] |
//! | **Read** | [`DeserializeGeometry`] | [`serde::Deserialize`] |
//!
//! # Traits for file formats
//!
//! | | Geometry | Properties |
//! |---|---|---|
//! | **Write** | — (takes any [`geo_traits::GeometryTrait`]) | [`SerializeProperties`] |
//! | **Read** | [`geo_traits::GeometryTrait`] | [`serde::Deserializer`] |
//!
//! Reading properties needs no trait of geoserde's own, since a flat row is
//! already a map in serde's data model: expose it as a
//! [`MapAccess`](serde::de::MapAccess). For its values, the built-in
//! deserializers in [`serde::de::value`] do not read a present value into an
//! `Option` field, so a format crate needs its own value deserializer to
//! support `Option` fields.
//!
//! # Cargo features
//!
//! * `geo-types` — [`DeserializeGeometry`] impls for [`geo_types`]. Enabled by default.

pub mod de;
pub mod ser;

pub use de::DeserializeGeometry;
pub use ser::SerializeProperties;
