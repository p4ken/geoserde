# GeoSerde

| [crates.io](https://crates.io/crates/geoserde) | [docs.rs](https://docs.rs/geoserde/latest/geoserde/) | [github](https://github.com/p4ken/geoserde) |

Serializer and deserializer for geospatial data.

GeoSerde can be used as an adapter between Serde and GeoZero.

| Serde       |     | GeoSerde            |     | GeoZero           |
| ----------- | --- | ------------------- | --- | ----------------- |
| Serialize   | --> | FeatureSerializer   | --> | FeatureProcessor  |
| Deserialize | <-- | FeatureDeserializer | <-- | GeozeroDatasource |

## Under development

The serializer currently only supports Point, Line, LineString or Polygon.

Deserializer is not yet implemented.

## Getting started

Add the dependency.

```shell
cargo add geoserde
```

## Cargo features

* `geozero` - Implements sink traits for geozero processors. Enabled by default.

## License

[MIT license](LICENSE)
