mod coord;
mod feat;
mod geom;
mod prop;

// CLEANUP-v0.6: `FeatureAccess` and `from_feature_iter` are unused by v0_6_2 (which has its own FeatureDeserializer / FeatureAccess); keep only `OwnedHeader`, `PropertyError`, `FeatureError` -- those are still referenced from v0_6_2
pub use feat::{FeatureAccess, FeatureError, PropertyError, from_feature_iter};

// pub struct FeatureIter {
//     header: Own

/// Owned clones of fbs header.
///
/// Why deep copy is needed:
/// - `flatgeobuf::FeatureIter::next()` takes `&mut self` which contains the header.
/// - `flatgeobuf::FgbFeature::header()` is private.
pub struct OwnedHeader {
    pub(crate) cols: Vec<OwnedColumn>,
    geom_type: flatgeobuf::GeometryType,
}

impl From<flatgeobuf::Header<'_>> for OwnedHeader {
    fn from(fbs: flatgeobuf::Header<'_>) -> Self {
        let cols = match fbs.columns() {
            Some(vec) => vec.into_iter().map(OwnedColumn::from).collect(),
            None => vec![],
        };
        let geom_type = fbs.geometry_type();
        Self { cols, geom_type }
    }
}

pub(crate) struct OwnedColumn {
    pub(crate) name: String,
    pub(crate) col_type: flatgeobuf::ColumnType,
}

impl From<flatgeobuf::Column<'_>> for OwnedColumn {
    fn from(col: flatgeobuf::Column<'_>) -> Self {
        Self {
            name: col.name().to_owned(),
            col_type: col.type_(),
        }
    }
}
