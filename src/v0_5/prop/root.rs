use serde::{
    ser::{Impossible, SerializeMap, SerializeStruct},
    Serialize, Serializer,
};

use crate::v0_5::prop::child::Child;

pub struct RootSerializer<M> {
    child: Child<M>,
}

impl<M> RootSerializer<M> {
    pub fn new<S: Serializer<SerializeMap = M>>(inner: S) -> Self {
        let table = inner.serialize_map(None).unwrap();
        Self {
            child: Child::new(table),
        }
    }
}

impl<'a, M: SerializeMap> Serializer for RootSerializer<M> {
    type Ok = M::Ok;
    type Error = M::Error;

    type SerializeSeq = Impossible<M::Ok, M::Error>;
    type SerializeTuple = Impossible<M::Ok, M::Error>;
    type SerializeTupleStruct = Impossible<M::Ok, M::Error>;
    type SerializeTupleVariant = Impossible<M::Ok, M::Error>;
    type SerializeMap = Impossible<M::Ok, M::Error>;
    type SerializeStruct = RootSerializer<M>;
    type SerializeStructVariant = Impossible<M::Ok, M::Error>;

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
        todo!()
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
        todo!()
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

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        todo!()
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

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        todo!()
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

impl<M: SerializeMap> SerializeStruct for RootSerializer<M> {
    type Ok = M::Ok;
    type Error = M::Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        (&mut self.child).serialize_field(key, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.child.into_table().end()
    }
}

// elem0,elem1

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Serialize)]
    struct Root {
        parent: Parent,
        number: i32,
    }

    #[derive(Serialize)]
    struct Parent {
        child: Child,
    }

    #[derive(Serialize)]
    struct Child {
        text: &'static str,
    }

    #[test]
    fn flatten_nested_key() {
        let root = Root {
            parent: Parent {
                child: Child { text: "hello" },
            },
            number: 2,
        };

        let mut buf = Vec::new();
        let mut json_ser = serde_json::Serializer::new(&mut buf);
        let ser = RootSerializer::new(&mut json_ser);
        root.serialize(ser).unwrap();
        assert_eq!(
            r#"{"parent.child.text":"hello","number":2}"#,
            String::from_utf8(buf).unwrap()
        );
    }
}
