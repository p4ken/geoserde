mod coord;
mod feat;
mod geom;
mod prop;

pub use feat::{from_feature_iter, FeatureAccess, FeatureError};

// pub struct FeatureIter {
//     header: Own

/// Owened clones of fbs header.
///
/// Why deep copy is needed:
/// - `flatgeobuf::FeatureIter::next()` takes `&mut self` which contains the header.
/// - `flatgeobuf::FgbFeature::header()` is private.
pub struct OwnedHeader {
    cols: Vec<OwnedColumn>,
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

struct OwnedColumn {
    name: String,
    col_type: flatgeobuf::ColumnType,
}

impl From<flatgeobuf::Column<'_>> for OwnedColumn {
    fn from(col: flatgeobuf::Column<'_>) -> Self {
        Self {
            name: col.name().to_owned(),
            col_type: col.type_(),
        }
    }
}
