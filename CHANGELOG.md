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

#### Serializing to FlatGeobuf

**Before (v0.5)**

```rust
let mut fgb = FgbWriter::create("layer", GeometryType::Unknown)?;
let mut ser = geoserde::FeatureSerializer::new(&mut fgb);
features.serialize(&mut ser)?;
```

**After (v0.6)**

Use `geoserde::fgb::FeatureSerializer` for streaming one feature at a time:

```rust
let fgb = FgbWriter::create("layer", GeometryType::Unknown)?;
let mut ser = geoserde::fgb::FeatureSerializer::new(fgb);
for feat in &features {
    ser.serialize_feature(&feat.geometry, feat)?;
}
let writer = ser.into_inner();
```

Or use `geoserde::fgb::LayerSerializer` when you need to sort columns across all features before writing (e.g. for consistent column ordering):

```rust
let mut layer = geoserde::fgb::LayerSerializer::new();
for feat in &features {
    layer.add_feature(&feat.geometry, feat);
}
// optionally reorder columns
let mut fgb = FgbWriter::create("layer", GeometryType::Unknown)?;
layer.write_features(&mut fgb)?;
```

#### Deserializing from FlatGeobuf

**Before (v0.5)**

There was no deserialization API in v0.5.

**After (v0.6)**

`geoserde::fgb::FeatureDeserializer` separates geometry and properties at the type level:

```rust
let mut de = geoserde::fgb::FeatureDeserializer::new(fgb_reader)?;
let (geom, props) = de.deserialize_feature::<geo_types::Point, MyProps>()?;
```

The geometry type must implement `geoserde::DeserializeGeometry`. Implementations for `geo_types::Point` and `geo_types::LineString` are provided when the `geo` feature is enabled. The properties type only needs `serde::Deserialize`, so `#[serde(flatten)]` works as expected.

#### Serializing to GeoJSON / WKT / other geozero formats

There is no equivalent API in v0.6. These output formats are not in scope for v0.6.
