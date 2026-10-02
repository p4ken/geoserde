# geoserde

[![crates.io](https://img.shields.io/crates/v/geoserde.svg)](https://crates.io/crates/geoserde)
[![docs.rs](https://img.shields.io/badge/_-docs.rs-slategray?logo=docsdotrs)](https://docs.rs/geoserde/)

geoserde is a framework for reading and writing geospatial data as Rust types.

* **Geometry** — any [geo-traits](https://crates.io/crates/geo-traits) type to write and any `DeserializeGeometry` type to read, such as [geo-types](https://crates.io/crates/geo-types).
* **Properties** — your own [serde](https://serde.rs/) structs, with nested structs, maps, arrays and enums flattened into the flat columns of an attribute table.

File formats are supported by separate crates, such as:

* [geoserde-fgb](https://docs.rs/geoserde-fgb) — FlatGeobuf
