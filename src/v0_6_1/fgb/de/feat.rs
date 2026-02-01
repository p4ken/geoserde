use std::fmt::Display;

use flatgeobuf::{ColumnType, FallibleStreamingIterator, FgbFeature};
use serde::de::{
    value::{EnumAccessDeserializer, MapAccessDeserializer},
    DeserializeOwned, DeserializeSeed, Error, IntoDeserializer, MapAccess, StdError,
};

use crate::v0_6_1::fgb::de::{
    geom::{GeometryAccess, GeometryError},
    OwnedHeader,
};

pub fn from_feature_iter<T, R, S>(
    mut fgb_iter: flatgeobuf::FeatureIter<R, S>,
) -> Result<Vec<T>, crate::v0_6_1::fgb::Error>
where
    T: DeserializeOwned,
    R: std::io::Read,
    flatgeobuf::FeatureIter<R, S>:
        FallibleStreamingIterator<Item = FgbFeature, Error = flatgeobuf::Error>,
{
    let mut features = vec![];
    let fgb_header = fgb_iter.header().into();
    while let Some(fgb_feat) = FallibleStreamingIterator::next(&mut fgb_iter)? {
        // T should implement DeserializeGeometry.
        // deserialize -> serde(with = "geoserde") -> deserialize_geometry
        let feat = T::deserialize(FeatureAccess::new(&fgb_header, fgb_feat).into_deserializer())?;
        features.push(feat);
    }
    Ok(features)
}

pub struct FeatureAccess<'de> {
    header: &'de OwnedHeader,

    // This field is None in cases:
    // - fgb feature has no geometry
    // - geometry has been deserialized once
    geom_de: Option<EnumAccessDeserializer<GeometryAccess<'de>>>,

    col_type: Option<ColumnType>,
    properties_buf: &'de [u8],
}

impl<'de> FeatureAccess<'de> {
    pub fn new(header: &'de OwnedHeader, feat: &'de FgbFeature) -> Self {
        Self {
            header,
            geom_de: feat
                .geometry()
                .map(|geom| GeometryAccess::new(geom, header.geom_type).into_deserializer()),
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

impl<'de> MapAccess<'de> for FeatureAccess<'de> {
    type Error = FeatureError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        // Deserialize geometry before any properties.
        if self.geom_de.is_some() {
            // The geometry field must be renamed to "geoserde::geometry".
            // This is because "geometry" may be used as a property name
            // and "::" is not used generally.
            return Ok(Some(
                seed.deserialize("geoserde::geometry".into_deserializer())
                    .map_err(FeatureError::Key)?,
            ));
        }

        // Deserialize properties.
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
        V: DeserializeSeed<'de>,
    {
        if let Some(geom) = self.geom_de.take() {
            return Ok(seed.deserialize(geom)?);
        }

        match self.col_type.unwrap() {
            ColumnType::Int => {
                let n = i32::from_le_bytes(self.take_prop(4)?.try_into().unwrap());
                seed.deserialize(n.into_deserializer())
            }
            ColumnType::String => {
                let len = u32::from_le_bytes(self.take_prop(4)?.try_into().unwrap()) as usize;
                let s = std::str::from_utf8(self.take_prop(len)?).map_err(PropertyError::from)?;
                seed.deserialize(s.into_deserializer())
            }
            x => panic!("{}", x.0),
        }

        // let column = &columns_meta.get(column_idx);
        // match column.type_() {
        //     ColumnType::Long => {
        //         finish = reader.property(
        //             column_idx,
        //             column.name(),
        //             &ColumnValue::Long(LittleEndian::read_i64(&bytes[offset..offset + 8])),
        //         )?;
        //         offset += size_of::<i64>();
        //     }
        //     ColumnType::ULong => {
        //         finish = reader.property(
        //             column_idx,
        //             column.name(),
        //             &ColumnValue::ULong(LittleEndian::read_u64(&bytes[offset..offset + 8])),
        //         )?;
        //         offset += size_of::<u64>();
        //     }
        //     ColumnType::Double => {
        //         finish = reader.property(
        //             column_idx,
        //             column.name(),
        //             &ColumnValue::Double(LittleEndian::read_f64(&bytes[offset..offset + 8])),
        //         )?;
        //         offset += size_of::<f64>();
        //     }
        //     ColumnType::Byte => {
        //         finish = reader.property(
        //             column_idx,
        //             column.name(),
        //             &ColumnValue::Byte(bytes[offset] as i8),
        //         )?;
        //         offset += size_of::<i8>();
        //     }
        //     ColumnType::UByte => {
        //         finish = reader.property(
        //             column_idx,
        //             column.name(),
        //             &ColumnValue::UByte(bytes[offset]),
        //         )?;
        //         offset += size_of::<u8>();
        //     }
        //     ColumnType::Bool => {
        //         finish = reader.property(
        //             column_idx,
        //             column.name(),
        //             &ColumnValue::Bool(bytes[offset] != 0),
        //         )?;
        //         offset += size_of::<u8>();
        //     }
        //     ColumnType::Short => {
        //         finish = reader.property(
        //             column_idx,
        //             column.name(),
        //             &ColumnValue::Short(LittleEndian::read_i16(&bytes[offset..offset + 2])),
        //         )?;
        //         offset += size_of::<i16>();
        //     }
        //     ColumnType::UShort => {
        //         finish = reader.property(
        //             column_idx,
        //             column.name(),
        //             &ColumnValue::UShort(LittleEndian::read_u16(&bytes[offset..offset + 2])),
        //         )?;
        //         offset += size_of::<u16>();
        //     }
        //     ColumnType::UInt => {
        //         finish = reader.property(
        //             column_idx,
        //             column.name(),
        //             &ColumnValue::UInt(LittleEndian::read_u32(&bytes[offset..offset + 4])),
        //         )?;
        //         offset += size_of::<u32>();
        //     }
        //     ColumnType::Float => {
        //         finish = reader.property(
        //             column_idx,
        //             column.name(),
        //             &ColumnValue::Float(LittleEndian::read_f32(&bytes[offset..offset + 4])),
        //         )?;
        //         offset += size_of::<f32>();
        //     }
        //     ColumnType::Json => {
        //         let len = LittleEndian::read_u32(&bytes[offset..offset + 4]) as usize;
        //         offset += size_of::<u32>();
        //         finish = reader.property(
        //             column_idx,
        //             column.name(),
        //             &ColumnValue::Json(
        //                 // JSON may be represented using UTF-8, UTF-16, or UTF-32. The default encoding is UTF-8.
        //                 str::from_utf8(&bytes[offset..offset + len]).map_err(|_| {
        //                     GeozeroError::Property("Invalid UTF-8 encoding".to_string())
        //                 })?,
        //             ),
        //         )?;
        //         offset += len;
        //     }
        //     ColumnType::DateTime => {
        //         let len = LittleEndian::read_u32(&bytes[offset..offset + 4]) as usize;
        //         offset += size_of::<u32>();
        //         finish = reader.property(
        //             column_idx,
        //             column.name(),
        //             &ColumnValue::DateTime(
        //                 // unsafe variant without UTF-8 checking would be faster...
        //                 str::from_utf8(&bytes[offset..offset + len]).map_err(|_| {
        //                     GeozeroError::Property("Invalid UTF-8 encoding".to_string())
        //                 })?,
        //             ),
        //         )?;
        //         offset += len;
        //     }
        //     ColumnType::Binary => {
        //         let len = LittleEndian::read_u32(&bytes[offset..offset + 4]) as usize;
        //         offset += size_of::<u32>();
        //         finish = reader.property(
        //             column_idx,
        //             column.name(),
        //             &ColumnValue::Binary(&bytes[offset..offset + len]),
        //         )?;
        //         offset += len;
        //     }
        //     ColumnType(_) => {}
        // }
    }
}

impl<'de> IntoDeserializer<'de, FeatureError> for FeatureAccess<'de> {
    type Deserializer = MapAccessDeserializer<Self>;

    fn into_deserializer(self) -> Self::Deserializer {
        MapAccessDeserializer::new(self)
    }
}

#[derive(Debug)]
pub enum FeatureError {
    Key(serde::de::value::Error),
    Geometry(GeometryError),
    Property(PropertyError),
    Deserialize(serde::de::value::Error),
}

impl Error for FeatureError {
    fn custom<T: Display>(msg: T) -> Self {
        Self::Deserialize(Error::custom(msg))
    }
}

impl StdError for FeatureError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(match self {
            Self::Key(e) => e,
            Self::Geometry(e) => e,
            Self::Deserialize(e) => e,
            _ => None?,
        })
    }
}

impl Display for FeatureError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Key(_) => write!(f, "attribute key deserializer failed"),
            Self::Geometry(_) => write!(f, "geometry deserializer failed"),
            Self::Property(_) => write!(f, "properties deserializer failed"),
            Self::Deserialize(_) => write!(f, "deserialize impl failed"),
        }
    }
}

impl From<GeometryError> for FeatureError {
    fn from(e: GeometryError) -> Self {
        Self::Geometry(e)
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

impl StdError for PropertyError {}

impl Display for PropertyError {
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
