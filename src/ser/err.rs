use std::{error::Error, fmt::Display};

/// The error in selialization by [FeatureSerializer](crate::FeatureSerializer),
/// [GeometrySerializer](crate::GeometrySerializer) and [PropertySerializer](crate::PropertySerializer).
#[derive(Debug, PartialEq)]
pub enum SerializeError<E> {
    SouceCaused(String),
    SinkCaused(E),
    NoGeometryField,
    InvalidFeatureStructure(&'static str),
    InvalidGeometryStructure {
        expected: Option<&'static str>,
        actual: &'static str,
    },
    UnsupportedPropertyStructure(&'static str),
    InvalidState,
}
impl<E: Error> serde::ser::Error for SerializeError<E> {
    fn custom<T: Display>(msg: T) -> Self {
        Self::SouceCaused(msg.to_string())
    }
}
impl<E: Display> Display for SerializeError<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use SerializeError::*;
        match self {
            SouceCaused(msg) => f.write_str(&msg),
            SinkCaused(e) => e.fmt(f),
            NoGeometryField => f.write_str("feature has no geometry field"),
            InvalidFeatureStructure(actual) => write!(f, "{} is not a feature struct", actual),
            InvalidGeometryStructure { expected, actual } => match expected {
                Some(expected) => write!(
                    f,
                    "expected {} but found {} in geometry container",
                    expected, actual
                ),
                None => write!(f, "unexpected {} in geometry container", actual),
            },
            UnsupportedPropertyStructure(actual) => {
                write!(f, "{} is not supported property", actual)
            }
            InvalidState => f.write_str("invalid internal state"),
        }
    }
}
impl<E: Error> Error for SerializeError<E> {}
