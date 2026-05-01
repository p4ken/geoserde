#![cfg(feature = "fgb")]

pub mod de;
pub mod ser;

// CLEANUP-v0.6: helper iterator API for the v0_6_1 serde-driven flow; v0_6_2 exposes FeatureDeserializer instead
pub use de::from_feature_iter;

// CLEANUP-v0.6: error wrapper used only by `from_feature_iter`; remove together with the helper above
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
