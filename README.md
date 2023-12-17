# geoserde

| [crates.io](https://crates.io/crates/geoserde) | [docs.rs](https://docs.rs/geoserde/latest/geoserde/) |

Serializer and deserializer for geospatial data.

The serializer currently only supports Point, Line, LineString or Polygon.

Deserializer is not yet implemented.

## Getting started

Add the dependency.

```shell
cargo add geoserde
```

## Examples

Serialize features (= geometry + properties) to json.

```rust
use geo_types::Point;
use geoserde::FeatureSerializer;
use geozero::geojson::GeoJsonWriter;
use serde::Serialize;

// feature
#[derive(Serialize)]
struct Station {
    loc: Point,         // geometry
    name: &'static str, // property
    europe: bool,       // property
}

fn main() -> anyhow::Result<()> {
    let features = vec![
        Station {
            loc: Point::new(51.5321, -0.1233),
            name: "King's Cross",
            europe: true,
        },
        Station {
            loc: Point::new(139.7661, 35.6812),
            name: "Tokyo",
            europe: false,
        },
    ];

    let mut buf = vec![];
    let mut json = GeoJsonWriter::new(&mut buf);
    let mut ser = FeatureSerializer::new(&mut json);
    features.serialize(&mut ser)?;

    println!("{}", std::str::from_utf8(&buf)?);
    Ok(())
}
```

## Cargo features

`geoserde` - Implement sink traits for geozero processors. Enabled by default.

## License

[MIT license](LICENSE)
