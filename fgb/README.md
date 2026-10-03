[![crates.io](https://img.shields.io/crates/v/geoserde-fgb.svg)](https://crates.io/crates/geoserde-fgb)
[![docs.rs](https://img.shields.io/badge/_-docs.rs-slategray?logo=docsdotrs)](https://docs.rs/geoserde-fgb/)

`geoserde-fgb` provides a serde-based API for reading and writing the [FlatGeobuf](https://crates.io/crates/flatgeobuf) format.

## Write

Write geometries and your own [serde](https://serde.rs/) structs as features.
The geometry can be any [geo-traits](https://crates.io/crates/geo-traits) type, such as [geo-types](https://crates.io/crates/geo-types).

```rust,no_run
use std::fs::File;
use std::io::BufWriter;

use serde::Serialize;

#[derive(Serialize)]
struct City {
    name: String,
    population: i64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ser = geoserde_fgb::LayerSerializer::new()?;
    let prop = City { name: "Tokyo".into(), population: 14_000_000 };
    let geom = geo_types::Point::new(139.7, 35.7);
    ser.serialize_feature(&geom, &prop)?;
    ser.write(BufWriter::new(File::create("cities.fgb")?))?;
    Ok(())
}
```

## Read

Read features into geo-types geometries and your own serde structs.

```rust,no_run
use std::fs::File;
use std::io::BufReader;

use serde::Deserialize;

#[derive(Deserialize)]
struct City {
    name: String,
    population: i64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = BufReader::new(File::open("cities.fgb")?);
    let mut de = geoserde_fgb::LayerDeserializer::new(file)?;
    // Pass the geometry type and the properties type to read into
    for feature in de.features::<geo_types::Point, City>() {
        let (geom, prop) = feature?;
        println!("{} {} {:?}", prop.name, prop.population, geom);
    }
    Ok(())
}
```

## Cargo features

* `geo-types` — Supports [geo-types](https://crates.io/crates/geo-types) geometries. Enabled by default.
