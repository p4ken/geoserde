use std::io::{Read, Seek};

use geoserde::DeserializeGeometry;
use serde::de::IntoDeserializer;

use crate::{Error, FeatureError, PropertyError};

/// Deserializes features (geometry + properties) from a FlatGeobuf source.
///
/// # Example
///
/// ```no_run
/// # use std::error::Error;
/// # fn main() -> Result<(), Box<dyn Error>> {
/// use geoserde_fgb::FeatureDeserializer;
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// struct Props { name: String }
///
/// let file = std::fs::File::open("example.fgb")?;
/// let reader = geoserde_fgb::flatgeobuf::FgbReader::open(
///     std::io::BufReader::new(file),
/// )?;
/// let mut de = FeatureDeserializer::new(reader)?;
/// for result in de.features::<geo_types::Point, Props>() {
///     let (geom, props) = result?;
/// }
/// # Ok(())
/// # }
/// ```
pub struct FeatureDeserializer<R> {
    fgb_iter: flatgeobuf::FeatureIter<R, flatgeobuf::Seekable>,
    header: OwnedHeader,
}

impl<R: Read + Seek> FeatureDeserializer<R> {
    /// Creates a new deserializer from an [`FgbReader`](flatgeobuf::FgbReader).
    ///
    /// All features are selected. The header is cloned internally because
    /// the underlying iterator requires mutable access.
    ///
    /// # Errors
    ///
    /// Returns [`Error`] if the FlatGeobuf header is invalid.
    pub fn new(fgb_reader: flatgeobuf::FgbReader<R>) -> Result<Self, Error> {
        let fgb_iter = fgb_reader.select_all()?;
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
        let prop = P::deserialize(prop_de).map_err(|source| Error::Feature {
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

/// Iterator adapter returned by [`FeatureDeserializer::features`].
pub struct Features<'a, R, G, P> {
    inner: &'a mut FeatureDeserializer<R>,
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
