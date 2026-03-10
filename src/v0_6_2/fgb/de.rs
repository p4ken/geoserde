use std::io::{Read, Seek};

use serde::de::IntoDeserializer;

use crate::{v0_6_1::fgb::de::OwnedHeader, v0_6_2::de::DeserializeGeometry};

pub struct FeatureDeserializer<R> {
    fgb_iter: flatgeobuf::FeatureIter<R, flatgeobuf::Seekable>,
    header: OwnedHeader,
}

impl<R: Read + Seek> FeatureDeserializer<R> {
    pub fn new(fgb_iter: flatgeobuf::FeatureIter<R, flatgeobuf::Seekable>) -> Self {
        let header = todo!();
        Self { fgb_iter, header }
    }

    pub fn deserialize_feature<G: DeserializeGeometry, P: serde::de::DeserializeOwned>(
        &mut self,
    ) -> Result<(G, P), flatgeobuf::Error> {
        let fgb_feat = flatgeobuf::FallibleStreamingIterator::next(&mut self.fgb_iter)?.unwrap();
        let geom = G::deserialize_geometry(fgb_feat.geometry_trait().unwrap().unwrap());
        let prop_de = FeatureAccess::new(&self.header, &fgb_feat).into_deserializer();
        let prop = P::deserialize(prop_de).unwrap();
        Ok((geom, prop))
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
    fn take_prop(&mut self, n: usize) -> Result<&[u8], crate::v0_6_1::fgb::de::PropertyError> {
        self.properties_buf
            .split_off(..n)
            .ok_or(crate::v0_6_1::fgb::de::PropertyError::Short)
    }
}

impl<'de> serde::de::MapAccess<'de> for FeatureAccess<'de> {
    type Error = crate::v0_6_1::fgb::de::FeatureError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
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
            .map_err(crate::v0_6_1::fgb::de::FeatureError::Key)?;
        self.col_type = Some(col.col_type);
        Ok(Some(key))
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        match self.col_type.unwrap() {
            flatgeobuf::ColumnType::Int => {
                let n = i32::from_le_bytes(self.take_prop(4)?.try_into().unwrap());
                seed.deserialize(n.into_deserializer())
            }
            flatgeobuf::ColumnType::String => {
                let len = u32::from_le_bytes(self.take_prop(4)?.try_into().unwrap()) as usize;
                let s = std::str::from_utf8(self.take_prop(len)?)
                    .map_err(crate::v0_6_1::fgb::de::PropertyError::from)?;
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

impl<'de> IntoDeserializer<'de, crate::v0_6_1::fgb::de::FeatureError> for FeatureAccess<'de> {
    type Deserializer = serde::de::value::MapAccessDeserializer<Self>;

    fn into_deserializer(self) -> Self::Deserializer {
        serde::de::value::MapAccessDeserializer::new(self)
    }
}
