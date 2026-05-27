//! Geometry deserialization from [`geo_traits`] sources.
//!
//! The central trait is [`DeserializeGeometry`], which converts a
//! [`GeometryTrait`](geo_traits::GeometryTrait) value into a concrete Rust type.
//! When the `geo` feature is enabled, implementations are provided for the
//! common [`geo_types`] geometry types.

mod geo;

/// Converts a [`GeometryTrait`](geo_traits::GeometryTrait) value into `Self`.
///
/// Implement this trait for your own geometry types so they can be
/// deserialized from any GIS source that exposes geometries through
/// [`geo_traits`].
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
///                 let c = p.coord().ok_or(GeometryTypeMismatch::new("Point"))?;
///                 Ok(Xy { x: c.x(), y: c.y() })
///             }
///             _ => Err(GeometryTypeMismatch::new("Point")),
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
#[derive(Debug)]
pub struct GeometryTypeMismatch {
    expected: &'static str,
}

impl GeometryTypeMismatch {
    /// Creates a new error indicating which geometry type was expected.
    pub fn new(expected: &'static str) -> Self {
        Self { expected }
    }
}

impl std::fmt::Display for GeometryTypeMismatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "expected {} geometry", self.expected)
    }
}

impl std::error::Error for GeometryTypeMismatch {}
