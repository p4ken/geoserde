# geoserde-fgb

[![crates.io](https://img.shields.io/crates/v/geoserde-fgb.svg)](https://crates.io/crates/geoserde-fgb)
[![docs.rs](https://img.shields.io/badge/_-docs.rs-slategray?logo=docsdotrs)](https://docs.rs/geoserde-fgb/)

geoserde-fgb reads and writes [FlatGeobuf](https://flatgeobuf.org/) features, with properties as your own [serde](https://serde.rs/) structs and geometries as [geo-types](https://crates.io/crates/geo-types).

## Write

```rust,no_run
use std::fs::File;
use std::io::BufWriter;

// Re-exported to match the version geoserde-fgb depends on
use geoserde_fgb::flatgeobuf;
use serde::Serialize;

#[derive(Serialize)]
struct City {
    name: String,
    population: i64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let writer = flatgeobuf::FgbWriter::create("cities", flatgeobuf::GeometryType::Point)?;
    let mut ser = geoserde_fgb::LayerSerializer::new(writer);
    let prop = City { name: "Tokyo".into(), population: 14_000_000 };
    let geom = geo_types::Point::new(139.7, 35.7);
    ser.serialize_feature(&geom, &prop)?;
    ser.into_inner().write(BufWriter::new(File::create("cities.fgb")?))?;
    Ok(())
}
```

## Read

```rust,no_run
use std::fs::File;
use std::io::BufReader;

use geoserde_fgb::flatgeobuf;
use serde::Deserialize;

#[derive(Deserialize)]
struct City {
    name: String,
    population: i64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = flatgeobuf::FgbReader::open(BufReader::new(File::open("cities.fgb")?))?;
    let mut de = geoserde_fgb::LayerDeserializer::new(reader)?;
    // Pass the geometry type and the properties type to read into
    for feature in de.features::<geo_types::Point, City>() {
        let (geom, prop) = feature?;
    }
    Ok(())
}
```

## Cargo features

* `geo-types` — Supports [geo-types](https://crates.io/crates/geo-types) geometries. Enabled by default.
