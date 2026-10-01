//! Reading features from FlatGeobuf.

use std::io::{Read, Seek};

use geoserde::DeserializeGeometry;
use geoserde::de::GeometryError;
use serde::de::IntoDeserializer;

/// Deserializes features (geometry + properties) from a FlatGeobuf source.
///
/// # Example
///
/// ```no_run
/// # use std::error::Error;
/// # fn main() -> Result<(), Box<dyn Error>> {
/// use geoserde_fgb::LayerDeserializer;
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// struct Props { name: String }
///
/// let file = std::fs::File::open("example.fgb")?;
/// let mut de = LayerDeserializer::new(std::io::BufReader::new(file))?;
/// for result in de.features::<geo_types::Point, Props>() {
///     let (geom, props) = result?;
/// }
/// # Ok(())
/// # }
/// ```
///
/// # Limitations
///
/// - Corrupted properties are not always detected. A column index that is not
///   in the header ends the properties of the feature, and the remaining bytes
///   are ignored, so wrong values can be returned without an error.
pub struct LayerDeserializer<R> {
    fgb_iter: flatgeobuf::FeatureIter<R, flatgeobuf::Seekable>,
    header: OwnedHeader,
}

impl<R: Read + Seek> LayerDeserializer<R> {
    /// Creates a new deserializer reading all features from `reader`.
    ///
    /// The header is read here. Features are read in many small reads, so
    /// wrap a [`File`](std::fs::File) in a [`BufReader`](std::io::BufReader).
    ///
    /// # Errors
    ///
    /// Returns [`Error::Fgb`] if the FlatGeobuf header is invalid.
    pub fn new(reader: R) -> Result<Self, Error> {
        let fgb_iter = flatgeobuf::FgbReader::open(reader)?.select_all()?;
        let header = fgb_iter.header().into();
        Ok(Self { fgb_iter, header })
    }

    /// Deserializes the next feature into a geometry and a properties struct.
    ///
    /// Returns `Ok(None)` when there are no more features.
    ///
    /// # Errors
    ///
    /// Returns [`Error`] on I/O, format, or deserialization failures.
    pub fn deserialize_feature<G: DeserializeGeometry, P: serde::de::DeserializeOwned>(
        &mut self,
    ) -> Result<Option<(G, P)>, Error> {
        let fgb_feat = match flatgeobuf::FallibleStreamingIterator::next(&mut self.fgb_iter)? {
            Some(f) => f,
            None => return Ok(None),
        };
        let geom_trait = fgb_feat.geometry_trait()?.ok_or(Error::MissingGeometry)?;
        let geom = G::deserialize_geometry(geom_trait)?;
        let mut prop_access = FeatureAccess::new(&self.header, fgb_feat);
        let prop_de = serde::de::value::MapAccessDeserializer::new(&mut prop_access);
        let prop = P::deserialize(prop_de).map_err(|source| ColumnError {
            column: prop_access.col.map(|c| c.name.clone()),
            source,
        })?;
        Ok(Some((geom, prop)))
    }

    /// Returns an iterator over deserialized features.
    pub fn features<G, P>(&mut self) -> Features<'_, R, G, P>
    where
        G: DeserializeGeometry,
        P: serde::de::DeserializeOwned,
    {
        Features {
            inner: self,
            _marker: std::marker::PhantomData,
        }
    }
}

/// Iterator adapter returned by [`LayerDeserializer::features`].
pub struct Features<'a, R, G, P> {
    inner: &'a mut LayerDeserializer<R>,
    _marker: std::marker::PhantomData<(G, P)>,
}

impl<R, G, P> Iterator for Features<'_, R, G, P>
where
    R: Read + Seek,
    G: DeserializeGeometry,
    P: serde::de::DeserializeOwned,
{
    type Item = Result<(G, P), Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.deserialize_feature().transpose()
    }
}

/// Low-level serde [`MapAccess`](serde::de::MapAccess) over a single
/// FlatGeobuf feature's property columns.
#[derive(Debug)]
pub(crate) struct FeatureAccess<'de> {
    header: &'de OwnedHeader,
    // Stays set on an error, telling which column failed.
    col: Option<&'de OwnedColumn>,
    properties_buf: &'de [u8],
}

impl<'de> FeatureAccess<'de> {
    /// Creates a new `FeatureAccess` from a header and a FlatGeobuf feature.
    pub(crate) fn new(header: &'de OwnedHeader, feat: &'de flatgeobuf::FgbFeature) -> Self {
        Self {
            header,
            col: None,
            properties_buf: match feat.fbs_feature().properties() {
                Some(fbs) => fbs.bytes(),
                None => &[],
            },
        }
    }
}

impl FeatureAccess<'_> {
    fn take_prop(&mut self, n: usize) -> Result<&[u8], PropertyError> {
        self.properties_buf
            .split_off(..n)
            .ok_or(PropertyError::Short)
    }
}

impl<'de> serde::de::MapAccess<'de> for FeatureAccess<'de> {
    type Error = FeatureError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        let col_index = match self.properties_buf.split_off(..2) {
            Some(bin) => u16::from_le_bytes(bin.try_into().unwrap()) as usize,
            None => return Ok(None),
        };
        let col = match self.header.cols.get(col_index) {
            Some(c) => c,
            None => return Ok(None),
        };
        self.col = Some(col);
        let key = seed
            .deserialize(col.name.as_str().into_deserializer())
            .map_err(FeatureError::Key)?;
        Ok(Some(key))
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        let value = match self.col.unwrap().col_type {
            flatgeobuf::ColumnType::Byte => {
                let v = self.take_prop(1)?[0] as i8;
                seed.deserialize(v.into_deserializer())
            }
            flatgeobuf::ColumnType::UByte => {
                let v = self.take_prop(1)?[0];
                seed.deserialize(v.into_deserializer())
            }
            flatgeobuf::ColumnType::Bool => {
                let v = self.take_prop(1)?[0] != 0;
                seed.deserialize(v.into_deserializer())
            }
            flatgeobuf::ColumnType::Short => {
                let n = i16::from_le_bytes(self.take_prop(2)?.try_into().unwrap());
                seed.deserialize(n.into_deserializer())
            }
            flatgeobuf::ColumnType::UShort => {
                let n = u16::from_le_bytes(self.take_prop(2)?.try_into().unwrap());
                seed.deserialize(n.into_deserializer())
            }
            flatgeobuf::ColumnType::UInt => {
                let n = u32::from_le_bytes(self.take_prop(4)?.try_into().unwrap());
                seed.deserialize(n.into_deserializer())
            }
            flatgeobuf::ColumnType::Float => {
                let n = f32::from_le_bytes(self.take_prop(4)?.try_into().unwrap());
                seed.deserialize(n.into_deserializer())
            }
            flatgeobuf::ColumnType::Int => {
                let n = i32::from_le_bytes(self.take_prop(4)?.try_into().unwrap());
                seed.deserialize(n.into_deserializer())
            }
            flatgeobuf::ColumnType::Long => {
                let n = i64::from_le_bytes(self.take_prop(8)?.try_into().unwrap());
                seed.deserialize(n.into_deserializer())
            }
            flatgeobuf::ColumnType::ULong => {
                let n = u64::from_le_bytes(self.take_prop(8)?.try_into().unwrap());
                seed.deserialize(n.into_deserializer())
            }
            flatgeobuf::ColumnType::Double => {
                let n = f64::from_le_bytes(self.take_prop(8)?.try_into().unwrap());
                seed.deserialize(n.into_deserializer())
            }
            flatgeobuf::ColumnType::String => {
                let len = u32::from_le_bytes(self.take_prop(4)?.try_into().unwrap()) as usize;
                let s = std::str::from_utf8(self.take_prop(len)?).map_err(PropertyError::from)?;
                seed.deserialize(s.into_deserializer())
            }
            flatgeobuf::ColumnType::Json | flatgeobuf::ColumnType::DateTime => {
                let len = u32::from_le_bytes(self.take_prop(4)?.try_into().unwrap()) as usize;
                let s = std::str::from_utf8(self.take_prop(len)?).map_err(PropertyError::from)?;
                seed.deserialize(s.into_deserializer())
            }
            flatgeobuf::ColumnType::Binary => {
                let len = u32::from_le_bytes(self.take_prop(4)?.try_into().unwrap()) as usize;
                let b = self.take_prop(len)?;
                seed.deserialize(b.into_deserializer())
            }
            x => Err(FeatureError::UnsupportedColumnType(x.0)),
        }?;
        self.col = None;
        Ok(value)
    }
}

/// Owned copy of a FlatGeobuf header.
///
/// A deep copy is required because the feature iterator's `next()` method
/// takes `&mut self` (which contains the header) while deserialization needs
/// shared access.
#[derive(Debug)]
pub(crate) struct OwnedHeader {
    cols: Vec<OwnedColumn>,
}

impl From<flatgeobuf::Header<'_>> for OwnedHeader {
    fn from(fbs: flatgeobuf::Header<'_>) -> Self {
        let cols = match fbs.columns() {
            Some(vec) => vec.into_iter().map(OwnedColumn::from).collect(),
            None => vec![],
        };
        Self { cols }
    }
}

#[derive(Debug)]
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

/// Error returned when reading features from a FlatGeobuf source.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// FlatGeobuf format error (corrupted file, invalid header, unsupported
    /// geometry type, etc.).
    Fgb(flatgeobuf::Error),
    /// The [`DeserializeGeometry`] impl returned an error.
    Geometry(GeometryError),
    /// The feature did not contain a geometry.
    MissingGeometry,
    /// Error from FlatGeobuf property deserialization via serde.
    Feature(ColumnError),
}

impl From<flatgeobuf::Error> for Error {
    fn from(e: flatgeobuf::Error) -> Self {
        Self::Fgb(e)
    }
}

impl From<GeometryError> for Error {
    fn from(e: GeometryError) -> Self {
        Self::Geometry(e)
    }
}

impl From<ColumnError> for Error {
    fn from(e: ColumnError) -> Self {
        Self::Feature(e)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fgb(_) => f.write_str("flatgeobuf format error"),
            Self::Geometry(_) => f.write_str("geometry deserialization failed"),
            Self::MissingGeometry => f.write_str("feature has no geometry"),
            Self::Feature(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Fgb(e) => Some(e),
            Self::Geometry(e) => Some(e),
            // Transparent, so that the column is not reported twice
            Self::Feature(e) => e.source(),
            Self::MissingGeometry => None,
        }
    }
}

/// Error returned when deserializing a feature's properties, with the column
/// being read.
#[derive(Debug, Clone)]
pub struct ColumnError {
    column: Option<String>,
    source: FeatureError,
}

impl ColumnError {
    /// Name of the column being read, or `None` if the error is not specific
    /// to a column (e.g. a missing field).
    pub fn column(&self) -> Option<&str> {
        self.column.as_deref()
    }

    /// The error raised while deserializing the properties.
    pub fn inner(&self) -> &FeatureError {
        &self.source
    }
}

impl std::fmt::Display for ColumnError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.column {
            Some(column) => write!(f, "failed to deserialize column `{column}`"),
            None => f.write_str("feature properties deserialization failed"),
        }
    }
}

impl std::error::Error for ColumnError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
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
