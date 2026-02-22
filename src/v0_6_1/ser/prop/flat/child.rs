use serde::{
    ser::{Error, Impossible, SerializeMap, SerializeSeq, SerializeStruct},
    Serialize, Serializer,
};

use crate::v0_6_1::ser::prop::flat::{
    node::{StringLike, Stringifier},
    FlattenError,
};

pub struct Child<M> {
    table: M,
    key: String,
    index: usize,
    value_seq: Vec<StringLike>,
}

impl<M> Child<M> {
    pub fn new(table: M) -> Self {
        Self {
            table,
            key: String::new(),
            index: 0,
            value_seq: Vec::new(),
        }
    }
    pub fn into_table(self) -> M {
        self.table
    }
}

impl<M: SerializeMap<Error: 'static>> SerializeSeq for &mut Child<M> {
    type Ok = ();
    type Error = FlattenError<M::Error>;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        match value.serialize(Stringifier) {
            Ok(text) => {
                self.value_seq.push(text);
                return Ok(());
            }
            // FIXME: Previous elements are not serialized if error
            Err(_) => self.value_seq.clear(),
        }

        let parent_key = self.key.clone();
        let parent_index = self.index;
        self.key = format!("{}[{}]", self.key, parent_index);
        value.serialize(&mut **self)?;
        self.index = parent_index + 1;
        self.key = parent_key;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        if self.value_seq.len() > 0 {
            let sep = ",";
            let value = self
                .value_seq
                .drain(..)
                .map(|text| text.to_string())
                .collect::<Vec<_>>()
                .join(sep);
            self.table.serialize_entry(&self.key, &value)?;
        }
        Ok(())
    }
}

impl<M: SerializeMap<Error: 'static>> SerializeStruct for &mut Child<M> {
    type Ok = ();
    type Error = FlattenError<M::Error>;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let parent = self.key.clone();

        match self.key.as_str() {
            "" => self.key = key.to_owned(), // TODO: Cow
            _ => self.key = format!("{}.{}", self.key, key),
        }
        value.serialize(&mut **self)?;

        self.key = parent;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, M: SerializeMap<Error: 'static>> SerializeMap for &mut Child<M> {
    type Ok = ();
    type Error = FlattenError<M::Error>;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let key = key.serialize(Stringifier).map_err(|_| M::Error::custom(""));
        todo!()
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a, M: SerializeMap<Error: 'static>> Serializer for &'a mut Child<M> {
    type Ok = ();
    type Error = FlattenError<M::Error>;

    type SerializeSeq = Self;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.table.serialize_entry(&self.key, &v)?;
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.table.serialize_entry(&self.key, v)?;
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.index = 0;
        Ok(self)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        todo!()
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        todo!()
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(self)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        todo!()
    }
}
