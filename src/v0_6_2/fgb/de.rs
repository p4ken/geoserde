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

    // pub fn with_bbox

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
                let s = std::str::from_utf8(self.take_prop(len)?)
                    .map_err(crate::v0_6_1::fgb::de::PropertyError::from)?;
                seed.deserialize(s.into_deserializer())
            }
            flatgeobuf::ColumnType::Json | flatgeobuf::ColumnType::DateTime => {
                let len = u32::from_le_bytes(self.take_prop(4)?.try_into().unwrap()) as usize;
                let s = std::str::from_utf8(self.take_prop(len)?)
                    .map_err(crate::v0_6_1::fgb::de::PropertyError::from)?;
                seed.deserialize(s.into_deserializer())
            }
            flatgeobuf::ColumnType::Binary => {
                let len = u32::from_le_bytes(self.take_prop(4)?.try_into().unwrap()) as usize;
                let b = self.take_prop(len)?;
                seed.deserialize(b.into_deserializer())
            }
            x => panic!("{}", x.0),
        }
    }
}

impl<'de> IntoDeserializer<'de, crate::v0_6_1::fgb::de::FeatureError> for FeatureAccess<'de> {
    type Deserializer = serde::de::value::MapAccessDeserializer<Self>;

    fn into_deserializer(self) -> Self::Deserializer {
        serde::de::value::MapAccessDeserializer::new(self)
    }
}
