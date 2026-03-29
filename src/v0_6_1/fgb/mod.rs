#![cfg(feature = "fgb")]

pub mod de;
pub mod ser;

pub use de::from_feature_iter;

#[derive(Debug)]
pub enum Error {
    Fgb(flatgeobuf::Error),
    De(de::FeatureError),
}

impl From<flatgeobuf::Error> for Error {
    fn from(e: flatgeobuf::Error) -> Self {
        Error::Fgb(e)
    }
}

impl From<de::FeatureError> for Error {
    fn from(e: de::FeatureError) -> Self {
        Error::De(e)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Fgb(e) => Some(e),
            Error::De(e) => Some(e),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Fgb(_) => write!(f, "flatgeobuf failed"),
            Error::De(_) => write!(f, "Deserializer failed"),
        }
    }
}
