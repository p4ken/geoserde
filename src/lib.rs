#![cfg_attr(all(doc, not(doctest)), feature(doc_auto_cfg))]

//! Geoserde is an adapter between geographic feature structs and GIS file formats.
//!
//! # Getting started
//!
//! ```sh
//! cargo add geoserde
//! ```
//!
//! # Cargo features
//!
//! * `geozero` - Implement geoserde sink for geozero processors. Enabled by default.
//!
//! # Examples
//!
//! ```
//! use geo_types::Point;
//! use geoserde::FeatureSerializer;
//! use geozero::geojson::GeoJsonWriter;
//! use serde::Serialize;
//!
//! // Print two features to the console in GeoJson format
//! fn main() -> anyhow::Result<()> {
//!     // If you want to write to a file, use BufWriter<File> instead
//!     let mut buf = vec![];
//!
//!     // Any format that has an implementation of geozero::FeatureProcessor can be used,
//!     // such as wkt, shp, fgb, etc. See also https://docs.rs/geozero/latest/geozero/
//!     let mut geojson = GeoJsonWriter::new(&mut buf);
//!
//!     // Serialize features to GeoJson format
//!     let mut ser = FeatureSerializer::new(&mut geojson);
//!     my_features().serialize(&mut ser)?;
//!
//!     println!("{}", std::str::from_utf8(&buf)?);
//!     Ok(())
//! }
//!
//! // Create feature array
//! fn my_features() -> impl Serialize {
//!     [
//!         Station {
//!             name: "King's Cross",
//!             europe: true,
//!             loc: Point::new(51.5321, -0.1233),
//!         },
//!         Station {
//!             name: "Tokyo",
//!             europe: false,
//!             loc: Point::new(139.7661, 35.6812),
//!         },
//!     ]
//! }
//!
//! // Geographic feature
//! #[derive(Serialize)]
//! struct Station {
//!     // Property
//!     name: &'static str,
//!
//!     // Property
//!     europe: bool,
//!
//!     // Geometry
//!     loc: Point,
//! }
//! ```

mod ser;

pub use crate::ser::*;
pub use geoserde_derive::Deserialize;
