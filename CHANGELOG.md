# Changelog

## v0.6.0 (2026-XX-XX)

### Migration guide from v0.5

v0.6 splits a feature into its geometry and its properties, and handles each
through a separate trait. Items are now under the `ser` and `de` modules
instead of the crate root.

| v0.5 | v0.6 |
|---|---|
| `FeatureSerializer` | No equivalent. Serialize geometry and properties separately. |
| `GeometrySerializer` | No equivalent. Formats read geometries through [`geo_traits::GeometryTrait`](https://docs.rs/geo-traits). |
| `PropertySerializer` | `ser::TableSerializer` |
| `PropertySink` | `ser::SerializeProperties` |
| `FeatureSink`, `GeometrySink` | No equivalent. |
| `SerializeError` | `ser::TableError` for properties, `de::GeometryTypeMismatch` for geometries |
| `geozero` feature | Removed. |
| — | `geo` feature (default): `de::DeserializeGeometry` impls for `geo_types` |

The crate now uses Rust edition 2024, which requires Rust 1.85 or later.

#### Keep the geometry out of the properties

`FeatureSerializer` picked the first `geo_types` field of a struct as the
geometry. v0.6 does not look for a geometry in the properties, so pass it to
the format on its own and skip it on the serde side.

**Before (v0.5)**

```rust
#[derive(Serialize)]
struct Station {
    name: String,
    loc: geo_types::Point,
}
```

**After (v0.6)**

```rust
#[derive(Serialize)]
struct Station {
    name: String,
    #[serde(skip)]
    loc: geo_types::Point,
}
```

#### Writing properties to your own format

`PropertySink` had one method per scalar type and received a column index.
`SerializeProperties` receives each flattened key with a `FieldValue`, and
returns a result from `end`.

**Before (v0.5)**

```rust
impl PropertySink for MyWriter {
    type Err = MyError;
    fn i32(&mut self, index: usize, key: &str, value: i32) -> Result<(), MyError> { ... }
    fn str(&mut self, index: usize, key: &str, value: &str) -> Result<(), MyError> { ... }
    // and so on for every scalar type
}

props.serialize(PropertySerializer::new(0, "", &mut writer))?;
```

**After (v0.6)**

```rust
use geoserde::ser::{FieldValue, SerializeProperties, TableSerializer};

impl SerializeProperties for &mut MyWriter {
    type Ok = ();
    type Error = MyError;
    fn serialize_property(
        &mut self,
        key: Cow<'static, str>,
        value: FieldValue<'_>,
    ) -> Result<(), MyError> {
        match value {
            FieldValue::I32(v) => { ... }
            FieldValue::Str(v) => { ... }
            _ => { ... }
        }
    }
    fn end(self) -> Result<(), MyError> { Ok(()) }
}

props.serialize(TableSerializer::new(&mut writer))?;
```

The root value must be a struct or a map. `TableError::Sink` wraps the error of
your sink, and `TableError::into_sink` takes it out.

#### Nested properties

Nested structs, maps and arrays were rejected with
`SerializeError::UnsupportedPropertyStructure`. They are now flattened into
keys such as `parent.child` and `items[0]`, and arrays of scalars are joined
into a string. Use `ser::FlattenOption` with `TableSerializer::with_option` to
change the separators. `ser::flatten_keys` lists the keys a value produces.

`None` and `()` are still skipped. A unit enum variant is now written under its
field name instead of the enum name.

#### Reading geometries

Deserialization is new in v0.6. Implement `de::DeserializeGeometry` to build
your geometry type from any `geo_traits::GeometryTrait` source. With the `geo`
feature, `geo_types::{Point, MultiPoint, LineString, MultiLineString, Polygon}`
implement it. Properties are read with plain `serde::Deserialize`.

#### geozero formats

The `geozero` feature and its implementations for geozero processors have been
removed. To write properties to a geozero processor, implement
`SerializeProperties` for it.
