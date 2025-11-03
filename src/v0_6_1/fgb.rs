use flatgeobuf::{ColumnType, FgbFeature};
use serde::{
    de::{value::StrDeserializer, MapAccess},
    Deserializer,
};

// pub struct LayerDeserializer {}
// impl LayerDeserializer {
//     pub fn new<R, S>(layer: FeatureIter<R, S>) -> Self {
//         Self {}
//     }
// }

pub struct FeatureDeserializer<'de> {
    cols: &'de [(String, ColumnType)],
    col_type: Option<ColumnType>,
    properties_buf: &'de [u8],
}
impl<'de> FeatureDeserializer<'de> {
    pub fn new(cols: &'de [(String, ColumnType)], feat: &'de FgbFeature) -> Self {
        Self {
            cols,
            col_type: None,
            properties_buf: match feat.fbs_feature().properties() {
                Some(fbs) => fbs.bytes(),
                None => &[],
            },
        }
    }
}
impl<'de, 'a> Deserializer<'de> for &'a mut FeatureDeserializer<'de> {
    type Error = serde::de::value::Error; // TODO

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.col_type {
            Some(ColumnType::Int) => visitor.visit_i32(i32::from_le_bytes(
                self.properties_buf
                    .split_off(..4)
                    .expect("i32 requires 4 bits")
                    .try_into()
                    .unwrap(),
            )),
            _ => todo!(),
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        if name == "__GeoSerdeGeometry" {
            return visitor.visit_newtype_struct(self);
        }
        todo!()
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_map(self)
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
        todo!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }
}

// struct PropertyAccess<'a, 'de: 'a> {
//     feat: &'a FeatureDeserializer<'de>,
//     col_idx: usize,
// }
impl<'de> MapAccess<'de> for FeatureDeserializer<'de> {
    type Error = serde::de::value::Error; // tmp

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        let col_index = match self.properties_buf.split_off(..2) {
            Some(bin) => u16::from_le_bytes(bin.try_into().unwrap()) as usize,
            None => return Ok(None),
        };
        let (col_name, col_type) = match self.cols.get(col_index) {
            Some(col) => col,
            None => return Ok(None),
        };
        let value = seed.deserialize(StrDeserializer::new(col_name))?;
        self.col_type = Some(*col_type);
        Ok(Some(value))
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        seed.deserialize(self)

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
