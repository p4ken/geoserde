use std::io::{Read, Seek};

use serde::de::IntoDeserializer;

use crate::de::DeserializeGeometry;
use crate::fgb::Error;

pub struct FeatureDeserializer<R> {
    fgb_iter: flatgeobuf::FeatureIter<R, flatgeobuf::Seekable>,
    header: OwnedHeader,
}

impl<R: Read + Seek> FeatureDeserializer<R> {
    pub fn new(fgb_reader: flatgeobuf::FgbReader<R>) -> Result<Self, Error> {
        let fgb_iter = fgb_reader.select_all()?;
        let header = fgb_iter.header().into();
        Ok(Self { fgb_iter, header })
    }

    pub fn deserialize_feature<G: DeserializeGeometry, P: serde::de::DeserializeOwned>(
        &mut self,
    ) -> Result<Option<(G, P)>, Error> {
        let fgb_feat = match flatgeobuf::FallibleStreamingIterator::next(&mut self.fgb_iter)? {
            Some(f) => f,
            None => return Ok(None),
        };
        let geom_trait = fgb_feat.geometry_trait()?.ok_or(Error::MissingGeometry)?;
        let geom = G::deserialize_geometry(geom_trait)?;
        let prop_de = FeatureAccess::new(&self.header, fgb_feat).into_deserializer();
        let prop = P::deserialize(prop_de)?;
        Ok(Some((geom, prop)))
    }
}

pub struct FeatureAccess<'de> {
    header: &'de OwnedHeader,
    col_type: Option<flatgeobuf::ColumnType>,
    properties_buf: &'de [u8],
}

impl<'de> FeatureAccess<'de> {
    pub fn new(header: &'de OwnedHeader, feat: &'de flatgeobuf::FgbFeature) -> Self {
        Self {
            header,
            col_type: None,
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
        let key = seed
            .deserialize(col.name.as_str().into_deserializer())
            .map_err(FeatureError::Key)?;
        self.col_type = Some(col.col_type);
        Ok(Some(key))
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        match self.col_type.unwrap() {
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
        }
    }
}

impl<'de> IntoDeserializer<'de, FeatureError> for FeatureAccess<'de> {
    type Deserializer = serde::de::value::MapAccessDeserializer<Self>;

    fn into_deserializer(self) -> Self::Deserializer {
        serde::de::value::MapAccessDeserializer::new(self)
    }
}

/// Owned clones of fbs header.
///
/// Why deep copy is needed:
/// - `flatgeobuf::FeatureIter::next()` takes `&mut self` which contains the header.
/// - `flatgeobuf::FgbFeature::header()` is private.
pub struct OwnedHeader {
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

#[derive(Debug)]
pub enum FeatureError {
    Key(serde::de::value::Error),
    Property(PropertyError),
    UnsupportedColumnType(u8),
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
            Self::Deserialize(e) => e,
            Self::Property(_) | Self::UnsupportedColumnType(_) => return None,
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

#[derive(Debug)]
pub enum PropertyError {
    Short,
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
