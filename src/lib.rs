#![cfg_attr(all(doc, not(doctest)), feature(doc_cfg))]

//! Geoserde is an adapter between Rust data structures and geospatial file formats.
//!
//! It bridges [`serde`]-based property serialization with geometry handling,
//! letting you read and write geospatial features as plain Rust structs.
//!
//! * **[`de`]** — Deserialize geometries from GIS sources into Rust types.
//! * **[`ser`]** — Serialize struct properties into flat key-value tables.
//!
//! # Getting started
//!
//! ```sh
//! cargo add geoserde
//! ```
//!
//! To read and write [FlatGeobuf](https://flatgeobuf.org/) files, use the
//! [`geoserde-fgb`](https://docs.rs/geoserde-fgb) crate.
//!
//! # Traits to implement
//!
//! A feature consists of a geometry and properties. Geoserde defines only two
//! traits of its own; the rest is covered by [`geo_traits`] and [`serde`].
//!
//! ## Your data types
//!
//! | | Geometry | Properties |
//! |---|---|---|
//! | **Write** | [`GeometryTrait`](geo_traits::GeometryTrait) | [`serde::Serialize`] |
//! | **Read** | [`DeserializeGeometry`] | [`serde::Deserialize`] |
//!
//! ## Formats
//!
//! | | Geometry | Properties |
//! |---|---|---|
//! | **Write** | (walks the `GeometryTrait`) | [`SerializeProperties`] |
//! | **Read** | (hands out a `GeometryTrait`) | [`serde::Deserializer`] |
//!
//! Reading properties needs no trait of geoserde's own. A flat row is already
//! a map in serde's data model, so implementing
//! [`MapAccess`](serde::de::MapAccess) and wrapping it in
//! [`MapAccessDeserializer`](serde::de::value::MapAccessDeserializer) is enough.
//!
//! # Cargo features
//!
//! * `geo` — [`DeserializeGeometry`] impls for [`geo_types`]. Enabled by default.

pub mod de;
pub mod ser;

pub use de::DeserializeGeometry;
pub use ser::SerializeProperties;
