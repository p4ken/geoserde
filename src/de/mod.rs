//! Geometry deserialization from [`geo_traits`] sources.
//!
//! The central trait is [`DeserializeGeometry`], which converts a
//! [`GeometryTrait`](geo_traits::GeometryTrait) value into a concrete Rust type.
//! When the `geo-types` feature is enabled, implementations are provided for the
//! common [`geo_types`] geometry types.
//!
//! # Geometry type conversion
//!
//! The [`geo_types`] implementations accept a source of the same geometry
//! type only, unless [`GeometryOptions`] allows otherwise.
//!
//! | Source \ Target                                                | [`Point`] | [`MultiPoint`] | [`LineString`] | [`MultiLineString`] | [`Polygon`] |
//! | -------------------------------------------------------------- | --------- | -------------- | -------------- | ------------------- | ----------- |
//! | [`Point`](geo_traits::GeometryType::Point)                     | ok        | promoted       | –              | –                   | –           |
//! | [`MultiPoint`](geo_traits::GeometryType::MultiPoint)           | –         | ok             | –              | –                   | –           |
//! | [`LineString`](geo_traits::GeometryType::LineString)           | –         | –              | ok             | promoted            | –           |
//! | [`MultiLineString`](geo_traits::GeometryType::MultiLineString) | –         | –              | –              | ok                  | –           |
//! | [`Polygon`](geo_traits::GeometryType::Polygon)                 | –         | –              | –              | –                   | ok          |
//!
//! [`Point`]: geo_types::Point
//! [`MultiPoint`]: geo_types::MultiPoint
//! [`LineString`]: geo_types::LineString
//! [`MultiLineString`]: geo_types::MultiLineString
//! [`Polygon`]: geo_types::Polygon
//!
//! - –: Always fails. So do `MultiPolygon`, `GeometryCollection`, `Rect`,
//!   `Triangle` and `Line` sources for every target.
//! - promoted: Fails by default. With
//!   [`set_promote_to_multi`](GeometryOptions::set_promote_to_multi), read as the
//!   multi type with one member.
//! - Failures are [`GeometryError`]s.
//! - Rings of a `Polygon` target are closed by [`geo_types::Polygon::new`].
//! - An empty point in a `Point` or `MultiPoint` source fails, because
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
/// Returns a [`GeometryError`] if the source geometry cannot be interpreted as
/// the target type under `options`.
///
/// # Example
///
/// ```
/// use geoserde::de::{DeserializeGeometry, GeometryError, GeometryOptions};
///
/// struct Xy { x: f64, y: f64 }
///
/// impl DeserializeGeometry for Xy {
///     fn deserialize_geometry<T: geo_traits::GeometryTrait<T = f64>>(
///         source: T,
///         _options: &GeometryOptions,
///     ) -> Result<Self, GeometryError> {
///         use geo_traits::{CoordTrait, GeometryType, PointTrait};
///         match source.as_type() {
///             GeometryType::Point(p) => {
///                 let c = p.coord().ok_or(GeometryError::custom("empty point"))?;
///                 Ok(Xy { x: c.x(), y: c.y() })
///             }
///             _ => Err(GeometryError::type_mismatch("Point", &source)),
///         }
///     }
/// }
/// ```
pub trait DeserializeGeometry: Sized {
    /// Deserialize `source` into `Self`.
    ///
    /// `options` tells which conversions between geometry types the caller
    /// allows. Impls may ignore the options that do not apply to `Self`.
    fn deserialize_geometry<T: geo_traits::GeometryTrait<T = f64>>(
        source: T,
        options: &GeometryOptions,
    ) -> Result<Self, GeometryError>;
}

/// Controls which conversions between geometry types
/// [`DeserializeGeometry`] allows.
///
/// By default, every conversion is disallowed, so the source must be of the
/// target type.
///
/// # Example
///
/// ```
/// use geoserde::de::GeometryOptions;
///
/// // Also read a Point as a MultiPoint with one member
/// let mut options = GeometryOptions::new();
/// options.set_promote_to_multi(true);
/// ```
#[derive(Debug, Clone)]
pub struct GeometryOptions {
    promote_to_multi: bool,
}

impl Default for GeometryOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl GeometryOptions {
    /// Creates a new `GeometryOptions` with the default options.
    pub fn new() -> Self {
        Self {
            // Following GDAL (PROMOTE_TO_MULTI) and serde, which require it to be explicit.
            promote_to_multi: false,
        }
    }

    /// Sets whether to read a single geometry as the multi type with one
    /// member. Defaults to `false`.
    ///
    /// See [Geometry type conversion](crate::de#geometry-type-conversion).
    pub fn set_promote_to_multi(&mut self, promote_to_multi: bool) {
        self.promote_to_multi = promote_to_multi;
    }

    /// Returns whether a single geometry may be read as the multi type.
    pub fn promote_to_multi(&self) -> bool {
        self.promote_to_multi
    }
}

#[derive(Debug, Clone)]
enum ErrorKind {
    TypeMismatch {
        expected: &'static str,
        found: &'static str,
    },
    Custom(String),
}

/// Error returned when a geometry cannot be deserialized.
///
/// The contents are private so that new kinds of failures can be added
/// without breaking [`DeserializeGeometry`] impls.
#[derive(Debug, Clone)]
pub struct GeometryError(ErrorKind);

impl GeometryError {
    /// Creates an error indicating which geometry type was expected, and which
    /// type `source` actually is.
    pub fn type_mismatch(expected: &'static str, source: &impl geo_traits::GeometryTrait) -> Self {
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
        Self(ErrorKind::TypeMismatch { expected, found })
    }

    /// Creates an error with a custom message, like
    /// [`serde::de::Error::custom`].
    pub fn custom(msg: impl std::fmt::Display) -> Self {
        Self(ErrorKind::Custom(msg.to_string()))
    }
}

impl std::fmt::Display for GeometryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            ErrorKind::TypeMismatch { expected, found } => {
                write!(f, "expected {expected}, found {found}")
            }
            ErrorKind::Custom(msg) => f.write_str(msg),
        }
    }
}

impl std::error::Error for GeometryError {}
