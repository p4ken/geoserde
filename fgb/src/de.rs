//! Reading features from FlatGeobuf.

use std::io::{Read, Seek};

use geoserde::DeserializeGeometry;
use geoserde::de::{GeometryError, GeometryOptions};
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
/// # Geometry types
///
/// Whether a geometry can be read depends on the [`DeserializeGeometry`] impl
/// of the target type. For `geo_types`, see [`geoserde::de`].
///
/// # Limitations
///
/// - Corrupted properties are not always detected. A column index that is not
///   in the header ends the properties of the feature, and the remaining bytes
///   are ignored, so wrong values can be returned without an error.
pub struct LayerDeserializer<R> {
    fgb_iter: flatgeobuf::FeatureIter<R, flatgeobuf::Seekable>,
    header: OwnedHeader,
    geometry_options: GeometryOptions,
}

impl<R> std::fmt::Debug for LayerDeserializer<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // `flatgeobuf::FeatureIter` does not implement `Debug`.
        f.debug_struct("LayerDeserializer")
            .field("header", &self.header)
            .field("geometry_options", &self.geometry_options)
            .finish_non_exhaustive()
    }
}

impl<R: Read + Seek> LayerDeserializer<R> {
    /// Creates a new deserializer reading all features from `reader`, with
    /// the default [`LayerOptions`].
    ///
    /// The header is read here. Features are read in many small reads, so
    /// wrap a [`File`](std::fs::File) in a [`BufReader`](std::io::BufReader).
    ///
    /// # Errors
    ///
    /// Returns [`Error::Fgb`] if the FlatGeobuf header is invalid.
    pub fn new(reader: R) -> Result<Self, Error> {
        Self::with_options(reader, LayerOptions::new())
    }

    /// Creates a new deserializer reading all features from `reader`, with
    /// the given options.
    ///
    /// See [`new`](Self::new) for details.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Fgb`] if the FlatGeobuf header is invalid.
    pub fn with_options(reader: R, options: LayerOptions) -> Result<Self, Error> {
        let fgb_iter = flatgeobuf::FgbReader::open(reader)?.select_all()?;
        let header = fgb_iter.header().into();
        Ok(Self {
            fgb_iter,
            header,
            geometry_options: options.geometry,
        })
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
        let geom = G::deserialize_geometry(geom_trait, &self.geometry_options)?;
        let mut prop_access = FeatureAccess::new(&self.header, fgb_feat);
        let prop_de = serde::de::value::MapAccessDeserializer::new(&mut prop_access);
        let prop = P::deserialize(prop_de).map_err(|source| PropertiesError {
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

/// Options for reading a FlatGeobuf layer.
///
/// # Example
///
/// ```no_run
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let options = geoserde_fgb::de::LayerOptions::new().promote_to_multi(true);
/// let file = std::fs::File::open("example.fgb")?;
/// let de = geoserde_fgb::LayerDeserializer::with_options(std::io::BufReader::new(file), options)?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct LayerOptions {
    geometry: GeometryOptions,
}

impl Default for LayerOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl LayerOptions {
    /// Creates a new `LayerOptions` with the default options.
    pub fn new() -> Self {
        Self {
            geometry: GeometryOptions::new(),
        }
    }

    /// Sets whether to read a single geometry as the multi type with one
    /// member, such as a `Point` as a `MultiPoint`. Defaults to `false`.
    ///
    /// See [`GeometryOptions::set_promote_to_multi`].
    pub fn promote_to_multi(mut self, promote_to_multi: bool) -> Self {
        self.geometry.set_promote_to_multi(promote_to_multi);
        self
    }
}

/// Iterator adapter returned by [`LayerDeserializer::features`].
pub struct Features<'a, R, G, P> {
    inner: &'a mut LayerDeserializer<R>,
    _marker: std::marker::PhantomData<(G, P)>,
}

// Manual impl to not require `G: Debug` and `P: Debug`.
impl<R, G, P> std::fmt::Debug for Features<'_, R, G, P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Features")
            .field("inner", &self.inner)
            .finish_non_exhaustive()
    }
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
    fn take_prop(&mut self, n: usize) -> Result<&[u8], DecodeError> {
        self.properties_buf
            .split_off(..n)
            .ok_or(DecodeError::Short)
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
                seed.deserialize(ValueDeserializer(v.into_deserializer()))
            }
            flatgeobuf::ColumnType::UByte => {
                let v = self.take_prop(1)?[0];
                seed.deserialize(ValueDeserializer(v.into_deserializer()))
            }
            flatgeobuf::ColumnType::Bool => {
                let v = self.take_prop(1)?[0] != 0;
                seed.deserialize(ValueDeserializer(v.into_deserializer()))
            }
            flatgeobuf::ColumnType::Short => {
                let n = i16::from_le_bytes(self.take_prop(2)?.try_into().unwrap());
                seed.deserialize(ValueDeserializer(n.into_deserializer()))
            }
            flatgeobuf::ColumnType::UShort => {
                let n = u16::from_le_bytes(self.take_prop(2)?.try_into().unwrap());
                seed.deserialize(ValueDeserializer(n.into_deserializer()))
            }
            flatgeobuf::ColumnType::UInt => {
                let n = u32::from_le_bytes(self.take_prop(4)?.try_into().unwrap());
                seed.deserialize(ValueDeserializer(n.into_deserializer()))
            }
            flatgeobuf::ColumnType::Float => {
                let n = f32::from_le_bytes(self.take_prop(4)?.try_into().unwrap());
                seed.deserialize(ValueDeserializer(n.into_deserializer()))
            }
            flatgeobuf::ColumnType::Int => {
                let n = i32::from_le_bytes(self.take_prop(4)?.try_into().unwrap());
                seed.deserialize(ValueDeserializer(n.into_deserializer()))
            }
            flatgeobuf::ColumnType::Long => {
                let n = i64::from_le_bytes(self.take_prop(8)?.try_into().unwrap());
                seed.deserialize(ValueDeserializer(n.into_deserializer()))
            }
            flatgeobuf::ColumnType::ULong => {
                let n = u64::from_le_bytes(self.take_prop(8)?.try_into().unwrap());
                seed.deserialize(ValueDeserializer(n.into_deserializer()))
            }
            flatgeobuf::ColumnType::Double => {
                let n = f64::from_le_bytes(self.take_prop(8)?.try_into().unwrap());
                seed.deserialize(ValueDeserializer(n.into_deserializer()))
            }
            flatgeobuf::ColumnType::String => {
                let len = u32::from_le_bytes(self.take_prop(4)?.try_into().unwrap()) as usize;
                let s = std::str::from_utf8(self.take_prop(len)?).map_err(DecodeError::from)?;
                seed.deserialize(ValueDeserializer(s.into_deserializer()))
            }
            flatgeobuf::ColumnType::Json | flatgeobuf::ColumnType::DateTime => {
                let len = u32::from_le_bytes(self.take_prop(4)?.try_into().unwrap()) as usize;
                let s = std::str::from_utf8(self.take_prop(len)?).map_err(DecodeError::from)?;
                seed.deserialize(ValueDeserializer(s.into_deserializer()))
            }
            flatgeobuf::ColumnType::Binary => {
                let len = u32::from_le_bytes(self.take_prop(4)?.try_into().unwrap()) as usize;
                let b = self.take_prop(len)?;
                seed.deserialize(ValueDeserializer(b.into_deserializer()))
            }
            x => Err(FeatureError::UnsupportedColumnType(x.0)),
        }?;
        self.col = None;
        Ok(value)
    }
}

/// Deserializer of a single property value.
///
/// The primitive deserializers of [`IntoDeserializer`] forward
/// `deserialize_option` and `deserialize_newtype_struct` to
/// `deserialize_any`, so they fail on `Option<T>` and newtype structs.
struct ValueDeserializer<D>(D);

impl<'de, D: serde::Deserializer<'de>> serde::Deserializer<'de> for ValueDeserializer<D> {
    type Error = D::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.0.deserialize_any(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        // FlatGeobuf leaves out a null property, so a present value is always `Some`.
        visitor.visit_some(self)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        // `StrDeserializer` reads a string into a unit variant.
        self.0.deserialize_enum(name, variants, visitor)
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf unit unit_struct seq tuple tuple_struct map struct
        identifier ignored_any
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
    Properties(PropertiesError),
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

impl From<PropertiesError> for Error {
    fn from(e: PropertiesError) -> Self {
        Self::Properties(e)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fgb(_) => f.write_str("flatgeobuf format error"),
            Self::Geometry(_) => f.write_str("geometry deserialization failed"),
            Self::MissingGeometry => f.write_str("feature has no geometry"),
            Self::Properties(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Fgb(e) => Some(e),
            Self::Geometry(e) => Some(e),
            // Transparent, so that the column is not reported twice
            Self::Properties(e) => e.source(),
            Self::MissingGeometry => None,
        }
    }
}

/// Error returned when deserializing a feature's properties, with the column
/// being read.
#[derive(Debug, Clone)]
pub struct PropertiesError {
    column: Option<String>,
    source: FeatureError,
}

impl PropertiesError {
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

impl std::fmt::Display for PropertiesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.column {
            Some(column) => write!(f, "failed to deserialize column `{column}`"),
            None => f.write_str("feature properties deserialization failed"),
        }
    }
}

impl std::error::Error for PropertiesError {
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
    Decode(DecodeError),
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
            Self::Decode(e) => e,
            Self::Deserialize(e) => e,
            Self::UnsupportedColumnType(_) => return None,
        })
    }
}

impl std::fmt::Display for FeatureError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Key(_) => write!(f, "attribute key deserializer failed"),
            Self::Decode(_) => write!(f, "properties deserializer failed"),
            Self::UnsupportedColumnType(c) => write!(f, "unsupported column type: {c}"),
            Self::Deserialize(_) => write!(f, "deserialize impl failed"),
        }
    }
}

impl From<DecodeError> for FeatureError {
    fn from(e: DecodeError) -> Self {
        Self::Decode(e)
    }
}

/// Error returned when reading raw property bytes from a FlatGeobuf feature.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum DecodeError {
    /// The property buffer ended before the expected number of bytes.
    Short,
    /// A string property contained invalid UTF-8.
    Utf8(std::str::Utf8Error),
}

impl std::error::Error for DecodeError {}

impl std::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Short => write!(f, "unexpected end of buffer"),
            Self::Utf8(e) => e.fmt(f),
        }
    }
}

impl From<std::str::Utf8Error> for DecodeError {
    fn from(e: std::str::Utf8Error) -> Self {
        Self::Utf8(e)
    }
}
