mod feat;
mod geom;

pub use feat::FeatureDeserializer;

// pub fn from_selected<R: std::io::Read + std::io::Seek>(selected: flatgeobuf::FeatureIter<R, flatgeobuf::Seekable>) {
// }

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
/// Reason to take deep copy:
/// - `flatgeobuf::FeatureIter::next()` takes mutable reference to self including header.
/// - `flatgeobuf::FgbFeature::header()` is private.
pub struct OwnedHeader {
    cols: Vec<OwnedColumn>,
    geom_type: flatgeobuf::GeometryType,
}
impl From<flatgeobuf::Header<'_>> for OwnedHeader {
    fn from(header: flatgeobuf::Header<'_>) -> Self {
        let cols = match header.columns() {
            Some(cols) => cols.into_iter().map(OwnedColumn::from).collect(),
            None => vec![],
        };
        Self {
            cols,
            geom_type: header.geometry_type(),
        }
    }
}

struct OwnedColumn {
    name: String,
    type_: flatgeobuf::ColumnType,
}
impl From<flatgeobuf::Column<'_>> for OwnedColumn {
    fn from(col: flatgeobuf::Column<'_>) -> Self {
        Self {
            name: col.name().to_owned(),
            type_: col.type_(),
        }
    }
}
