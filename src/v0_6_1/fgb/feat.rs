use flatgeobuf::{ColumnType, FgbFeature};
use serde::de::{
    value::{I32Deserializer, StrDeserializer},
    Error, MapAccess,
};

use crate::v0_6_1::fgb::{geom::GeometryDeserializer, OwnedHeader};

pub struct FeatureDeserializer<'de> {
    header: &'de OwnedHeader,
    // This field is None in cases:
    // - fgb feature has no geometry
    // - geometry has been deserialized once
    geom_de: Option<GeometryDeserializer<'de>>,
    col_type: Option<ColumnType>,
    properties_buf: &'de [u8],
}
impl<'de> FeatureDeserializer<'de> {
    pub fn new(header: &'de OwnedHeader, feat: &'de FgbFeature) -> Self {
        Self {
            header,
            geom_de: feat.geometry().map(GeometryDeserializer::new),
            col_type: None,
            properties_buf: match feat.fbs_feature().properties() {
                Some(fbs) => fbs.bytes(),
                None => &[],
            },
        }
    }
}
impl FeatureDeserializer<'_> {
    fn take_prop(&mut self, n: usize) -> Result<&[u8], serde::de::value::Error> {
        match self.properties_buf.split_off(..n) {
            Some(slice) => Ok(slice),
            None => Err(Error::custom("properties buffer out of bounds")),
        }
    }
}
impl<'de> MapAccess<'de> for FeatureDeserializer<'de> {
    type Error = serde::de::value::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        // Deserialize geometry before any property
        if self.geom_de.is_some() {
            // The geometry field must be renamed to "geoserde::geometry".
            // This is because "geometry" may be used as a property name
            // and "::" is not used in normal property names.
            return Ok(Some(
                seed.deserialize(StrDeserializer::new("geoserde::geometry"))?,
            ));
        }

        let col_index = match self.properties_buf.split_off(..2) {
            Some(bin) => u16::from_le_bytes(bin.try_into().unwrap()) as usize,
            None => return Ok(None),
        };
        let col = match self.header.cols.get(col_index) {
            Some(c) => c,
            None => return Ok(None),
        };
        let value = seed.deserialize(StrDeserializer::new(&col.name))?;
        self.col_type = Some(col.col_type);
        Ok(Some(value))
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        if let Some(geom) = self.geom_de.take() {
            return seed.deserialize(geom);
        }

        match self.col_type.unwrap() {
            ColumnType::Int => seed.deserialize(I32Deserializer::new(i32::from_le_bytes(
                self.take_prop(4)?.try_into().unwrap(),
            ))),
            ColumnType::String => {
                let len = u32::from_le_bytes(self.take_prop(4)?.try_into().unwrap()) as usize;
                let s = std::str::from_utf8(self.take_prop(len)?).map_err(Error::custom)?;
                seed.deserialize(StrDeserializer::new(s))
            }
            x => panic!("{}", x.0),
        }

        // let column = &columns_meta.get(column_idx);
        // match column.type_() {
        //     ColumnType::Int => {
        //         finish = reader.property(
        //             column_idx,
        //             column.name(),
        //             &ColumnValue::Int(LittleEndian::read_i32(&bytes[offset..offset + 4])),
        //         )?;
        //         offset += size_of::<i32>();
        //     }
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
        //     ColumnType::String => {
        //         let len = LittleEndian::read_u32(&bytes[offset..offset + 4]) as usize;
        //         offset += size_of::<u32>();
        //         finish = reader.property(
        //             column_idx,
        //             column.name(),
        //             &ColumnValue::String(
        //                 // unsafe variant without UTF-8 checking would be faster...
        //                 str::from_utf8(&bytes[offset..offset + len]).map_err(|_| {
        //                     GeozeroError::Property("Invalid UTF-8 encoding".to_string())
        //                 })?,
        //             ),
        //         )?;
        //         offset += len;
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
