use std::convert::Infallible;

use crate::de::GeometryTypeMismatch;
use crate::fgb::de::FeatureError;
use crate::ser::{SourceError, TableError};

/// Error type for FlatGeobuf operations.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// FlatGeobuf format error (corrupted file, invalid header, missing index, etc.).
    Fgb(flatgeobuf::Error),
    /// Error during geozero geometry processing (writing).
    Geozero(flatgeobuf::geozero::error::GeozeroError),
    /// The user's [`DeserializeGeometry`](crate::de::DeserializeGeometry) impl
    /// returned a geometry type mismatch.
    GeometryType(GeometryTypeMismatch),
    /// The feature did not contain a geometry.
    MissingGeometry,
    /// An unsupported `ColumnType` was encountered in the FlatGeobuf properties.
    UnsupportedColumnType(u8),
    /// Error from FlatGeobuf property deserialization via serde.
    Feature(FeatureError),
    /// The properties could not be flattened into a table.
    Table(TableError<Infallible>),
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
        match e {
            TableError::Root => Self::Table(TableError::Root),
            TableError::Key(k) => Self::Table(TableError::Key(k)),
            TableError::Sink(inner) => inner,
        }
    }
}

impl From<TableError<SourceError>> for Error {
    fn from(e: TableError<SourceError>) -> Self {
        match e {
            TableError::Root => Self::Table(TableError::Root),
            TableError::Key(k) => Self::Table(TableError::Key(k)),
            TableError::Sink(inner) => Self::Source(inner),
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
            Self::UnsupportedColumnType(c) => write!(f, "unsupported column type: {c}"),
            Self::Feature(_) => f.write_str("feature properties deserialization failed"),
            Self::Table(_) => f.write_str("properties serialization failed"),
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
            Self::MissingGeometry | Self::UnsupportedColumnType(_) => None,
        }
    }
}

impl serde::ser::Error for Error {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Self::Source(msg.to_string().into())
    }
}
