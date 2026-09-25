# Changelog

## v0.6.0 (2026-XX-XX)

### Removed

The following items, previously exposed at the crate root, have been removed.

| Removed item | Kind |
|---|---|
| `FeatureSerializer` | struct |
| `GeometrySerializer` | struct |
| `PropertySerializer` | struct |
| `SerializeError` | enum |
| `FeatureSink` | trait |
| `GeometrySink` | trait |
| `PropertySink` | trait |

The `geozero` feature flag has also been removed along with these items.

### Migration guide

#### FlatGeobuf support moved to `geoserde-fgb`

FlatGeobuf reading and writing is provided by the separate
[geoserde-fgb](https://crates.io/crates/geoserde-fgb) crate, so that updates to
`flatgeobuf` do not require a breaking release of `geoserde`.

```sh
cargo add geoserde-fgb
```

#### Serializing to FlatGeobuf

**Before (v0.5)**

```rust
let mut fgb = FgbWriter::create("layer", GeometryType::Unknown)?;
let mut ser = geoserde::FeatureSerializer::new(&mut fgb);
features.serialize(&mut ser)?;
```

**After (v0.6)**

Use `geoserde_fgb::FeatureSerializer` for streaming one feature at a time:

```rust
let fgb = FgbWriter::create("layer", GeometryType::Unknown)?;
let mut ser = geoserde_fgb::FeatureSerializer::new(fgb);
for feat in &features {
    ser.serialize_feature(&feat.geometry, feat)?;
}
let writer = ser.into_inner();
```

#### Deserializing from FlatGeobuf

**Before (v0.5)**

There was no deserialization API in v0.5.

**After (v0.6)**

`geoserde_fgb::FeatureDeserializer` separates geometry and properties at the type level:

```rust
let mut de = geoserde_fgb::FeatureDeserializer::new(fgb_reader)?;
let (geom, props) = de.deserialize_feature::<geo_types::Point, MyProps>()?;
```

The geometry type must implement `geoserde::DeserializeGeometry`. Implementations for `geo_types::Point` and `geo_types::LineString` are provided when the `geo` feature is enabled. The properties type only needs `serde::Deserialize`, so `#[serde(flatten)]` works as expected.

#### Serializing to GeoJSON / WKT / other geozero formats

There is no equivalent API in v0.6. These output formats are not in scope for v0.6.
