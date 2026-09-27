use std::convert::Infallible;

use geoserde::de::GeometryTypeMismatch;
use geoserde::ser::{FieldValue, SourceError, TableError};

/// Error type for FlatGeobuf operations.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// FlatGeobuf format error (corrupted file, invalid header, missing index, etc.).
    Fgb(flatgeobuf::Error),
    /// Error during geozero geometry processing (writing).
    Geozero(flatgeobuf::geozero::error::GeozeroError),
    /// The user's [`DeserializeGeometry`](geoserde::de::DeserializeGeometry) impl
    /// returned a geometry type mismatch.
    GeometryType(GeometryTypeMismatch),
    /// The feature did not contain a geometry.
    MissingGeometry,
    /// Error from FlatGeobuf property deserialization via serde.
    Feature(FeatureError),
    /// The properties could not be flattened into a table.
    Table(TableError<Infallible>),
    /// A property value has no corresponding FlatGeobuf column type.
    UnsupportedFieldValue(FieldValue<'static>),
    /// A generic error originating from the user's `Serialize` implementation.
    Source(SourceError),
}

impl From<flatgeobuf::Error> for Error {
    fn from(e: flatgeobuf::Error) -> Self {
        Self::Fgb(e)
    }
}

impl From<flatgeobuf::geozero::error::GeozeroError> for Error {
    fn from(e: flatgeobuf::geozero::error::GeozeroError) -> Self {
        Self::Geozero(e)
    }
}

impl From<GeometryTypeMismatch> for Error {
    fn from(e: GeometryTypeMismatch) -> Self {
        Self::GeometryType(e)
    }
}

impl From<FeatureError> for Error {
    fn from(e: FeatureError) -> Self {
        Self::Feature(e)
    }
}

impl From<TableError<Error>> for Error {
    fn from(e: TableError<Error>) -> Self {
        match e.into_sink() {
            Ok(inner) => inner,
            Err(table) => Self::Table(table),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fgb(_) => f.write_str("flatgeobuf format error"),
            Self::Geozero(_) => f.write_str("geozero processing failed"),
            Self::GeometryType(_) => f.write_str("geometry type mismatch"),
            Self::MissingGeometry => f.write_str("feature has no geometry"),
            Self::Feature(_) => f.write_str("feature properties deserialization failed"),
            Self::Table(_) => f.write_str("properties serialization failed"),
            Self::UnsupportedFieldValue(v) => write!(f, "unsupported field value: {v:?}"),
            Self::Source(_) => f.write_str("upstream serialize impl caused"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Fgb(e) => Some(e),
            Self::Geozero(e) => Some(e),
            Self::GeometryType(e) => Some(e),
            Self::Feature(e) => Some(e),
            Self::Table(e) => Some(e),
            Self::Source(e) => Some(e),
            Self::MissingGeometry | Self::UnsupportedFieldValue(_) => None,
        }
    }
}

impl serde::ser::Error for Error {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Self::Source(msg.to_string().into())
    }
}

/// Error returned when deserializing a single FlatGeobuf feature's properties.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum FeatureError {
    /// The column key could not be deserialized.
    Key(serde::de::value::Error),
    /// The property buffer was truncated or contained invalid data.
    Property(PropertyError),
    /// The column type is not supported by this deserializer.
    UnsupportedColumnType(u8),
    /// A `serde::Deserialize` implementation returned an error.
    Deserialize(serde::de::value::Error),
}

impl serde::de::Error for FeatureError {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Self::Deserialize(serde::de::Error::custom(msg))
    }
}

impl std::error::Error for FeatureError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(match self {
            Self::Key(e) => e,
            Self::Property(e) => e,
            Self::Deserialize(e) => e,
            Self::UnsupportedColumnType(_) => return None,
        })
    }
}

impl std::fmt::Display for FeatureError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Key(_) => write!(f, "attribute key deserializer failed"),
            Self::Property(_) => write!(f, "properties deserializer failed"),
            Self::UnsupportedColumnType(c) => write!(f, "unsupported column type: {c}"),
            Self::Deserialize(_) => write!(f, "deserialize impl failed"),
        }
    }
}

impl From<PropertyError> for FeatureError {
    fn from(e: PropertyError) -> Self {
        Self::Property(e)
    }
}

/// Error returned when reading raw property bytes from a FlatGeobuf feature.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum PropertyError {
    /// The property buffer ended before the expected number of bytes.
    Short,
    /// A string property contained invalid UTF-8.
    Utf8(std::str::Utf8Error),
}

impl std::error::Error for PropertyError {}

impl std::fmt::Display for PropertyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Short => write!(f, "unexpected end of buffer"),
            Self::Utf8(e) => e.fmt(f),
        }
    }
}

impl From<std::str::Utf8Error> for PropertyError {
    fn from(e: std::str::Utf8Error) -> Self {
        Self::Utf8(e)
    }
}
