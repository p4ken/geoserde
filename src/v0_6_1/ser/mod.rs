// CLEANUP-v0.6: `feat::AsFeature` is part of the v0_6_1 serde-driven feature pipeline; v0_6_2 takes (geometry, properties) explicitly and does not need this trait
mod feat;
mod prop;

// CLEANUP-v0.6: re-export of the unused AsFeature trait; remove together with `feat`
pub use feat::AsFeature;
pub use prop::*;

#[derive(Debug)]
pub struct SourceError(String);

impl From<String> for SourceError {
    fn from(string: String) -> Self {
        Self(string)
    }
}

impl std::fmt::Display for SourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for SourceError {}
