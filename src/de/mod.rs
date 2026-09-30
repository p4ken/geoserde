//! Geometry deserialization from [`geo_traits`] sources.
//!
//! The central trait is [`DeserializeGeometry`], which converts a
//! [`GeometryTrait`](geo_traits::GeometryTrait) value into a concrete Rust type.
//! When the `geo-types` feature is enabled, implementations are provided for the
//! common [`geo_types`] geometry types.
//!
//! # Geometry type conversion
//!
//! The [`geo_types`] implementations also accept a source of a different
//! geometry type.
//!
//! | Source \ Target                                                | [`Point`]     | [`MultiPoint`] | [`LineString`] | [`MultiLineString`]      | [`Polygon`]                 |
//! | -------------------------------------------------------------- | ------------- | -------------- | -------------- | ------------------------ | --------------------------- |
//! | [`Point`](geo_traits::GeometryType::Point)                     | ok            | ok (one point) | –              | –                        | –                           |
//! | [`MultiPoint`](geo_traits::GeometryType::MultiPoint)           | only point \* | ok             | ok             | ok (one line)            | ok (exterior)               |
//! | [`LineString`](geo_traits::GeometryType::LineString)           | –             | ok             | ok             | ok (one line)            | ok (exterior)               |
//! | [`MultiLineString`](geo_traits::GeometryType::MultiLineString) | –             | only line \*   | only line \*   | ok                       | ok (first line is exterior) |
//! | [`Polygon`](geo_traits::GeometryType::Polygon)                 | –             | exterior \*    | exterior \*    | ok (exterior, interiors) | ok                          |
//!
//! [`Point`]: geo_types::Point
//! [`MultiPoint`]: geo_types::MultiPoint
//! [`LineString`]: geo_types::LineString
//! [`MultiLineString`]: geo_types::MultiLineString
//! [`Polygon`]: geo_types::Polygon
//!
//! - –: Always fails. So do `MultiPolygon`, `GeometryCollection`, `Rect`,
//!   `Triangle` and `Line` sources for every target.
//! - \*: Fails unless there is exactly one point or line, or the polygon has
//!   an exterior and no interiors, so that no coordinates are dropped.
//! - Failures are [`GeometryTypeMismatch`]. The number of coordinates is not
//!   checked otherwise, so a single point becomes a one-coordinate
//!   `LineString`, an empty `Polygon` becomes an empty `MultiLineString`, and
//!   an empty `MultiLineString` becomes an empty `Polygon`.
//! - Rings of a `Polygon` target are closed by [`geo_types::Polygon::new`].
//! - An empty point in a `Point` or `MultiPoint` source panics, because
//!   [`geo_types`] cannot represent it.

mod geo;

/// Converts a [`GeometryTrait`](geo_traits::GeometryTrait) value into `Self`.
///
/// Implement this trait for your own geometry types so they can be
/// deserialized from any GIS source that exposes geometries through
/// [`geo_traits`]. The source lets you look into an existing geometry via
/// [`GeometryTrait`](geo_traits::GeometryTrait), and this trait adds the other
/// half, building `Self` from it, much as serde pairs `Deserialize` with
/// `Serialize`.
///
/// # Errors
///
/// Returns [`GeometryTypeMismatch`] if the source geometry cannot be
/// interpreted as the target type.
///
/// # Example
///
/// ```
/// use geoserde::de::{DeserializeGeometry, GeometryTypeMismatch};
///
/// struct Xy { x: f64, y: f64 }
///
/// impl DeserializeGeometry for Xy {
///     fn deserialize_geometry<T: geo_traits::GeometryTrait<T = f64>>(
///         source: T,
///     ) -> Result<Self, GeometryTypeMismatch> {
///         use geo_traits::{CoordTrait, GeometryType, PointTrait};
///         match source.as_type() {
///             GeometryType::Point(p) => {
///                 let c = p.coord().ok_or(GeometryTypeMismatch::new("Point", &source))?;
///                 Ok(Xy { x: c.x(), y: c.y() })
///             }
///             _ => Err(GeometryTypeMismatch::new("Point", &source)),
///         }
///     }
/// }
/// ```
pub trait DeserializeGeometry: Sized {
    /// Deserialize `source` into `Self`.
    fn deserialize_geometry<T: geo_traits::GeometryTrait<T = f64>>(
        source: T,
    ) -> Result<Self, GeometryTypeMismatch>;
}

/// Error returned when a geometry's type does not match the expected target.
#[derive(Debug, Clone)]
pub struct GeometryTypeMismatch {
    expected: &'static str,
    found: &'static str,
}

impl GeometryTypeMismatch {
    /// Creates a new error indicating which geometry type was expected, and
    /// which type `source` actually is.
    pub fn new(expected: &'static str, source: &impl geo_traits::GeometryTrait) -> Self {
        use geo_traits::GeometryType;
        let found = match source.as_type() {
            GeometryType::Point(_) => "Point",
            GeometryType::LineString(_) => "LineString",
            GeometryType::Polygon(_) => "Polygon",
            GeometryType::MultiPoint(_) => "MultiPoint",
            GeometryType::MultiLineString(_) => "MultiLineString",
            GeometryType::MultiPolygon(_) => "MultiPolygon",
            GeometryType::GeometryCollection(_) => "GeometryCollection",
            GeometryType::Rect(_) => "Rect",
            GeometryType::Triangle(_) => "Triangle",
            GeometryType::Line(_) => "Line",
        };
        Self { expected, found }
    }
}

impl std::fmt::Display for GeometryTypeMismatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "expected {}, found {}", self.expected, self.found)
    }
}

impl std::error::Error for GeometryTypeMismatch {}
