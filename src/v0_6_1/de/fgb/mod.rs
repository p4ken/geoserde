mod coord;
mod feat;
mod geom;

pub use feat::FeatureDeserializer;

// pub struct DeserializerIter<'a, R> {
//     fgb_iter: &'a mut flatgeobuf::FeatureIter<R, flatgeobuf::Seekable>,
//     header: OwnedHeader,
// }
// impl<'a, R: std::io::Read + std::io::Seek> DeserializerIter<'a, R> {
//     pub fn new(
//         fgb_iter: &'a mut flatgeobuf::FeatureIter<R, flatgeobuf::Seekable>,
//     ) -> Result<Self, flatgeobuf::Error> {
//         let header = fgb_iter.header().into();
//         Ok(Self { fgb_iter, header })
//     }
// }
// impl<'a, R: std::io::Read + std::io::Seek> Iterator for DeserializerIter<'a, R> {
//     type Item = FeatureDeserializer<'a>;

//     fn next(&mut self) -> Option<Self::Item> {
//         let fgb_feat = self.fgb_iter.next().transpose()?.unwrap();
//         let de = FeatureDeserializer::new(&self.header, fgb_feat);
//         Some(de)
//     }
// }

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
