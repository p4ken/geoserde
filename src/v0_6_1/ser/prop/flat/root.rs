use serde::{
    ser::{Error, Impossible, SerializeMap, SerializeStruct, StdError},
    Serialize, Serializer,
};

use crate::v0_6_1::ser::prop::flat::child::Child;

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

impl<M: SerializeMap<Error: 'static>> Serializer for RootSerializer<M> {
    type Ok = M::Ok;
    type Error = FlattenError<M::Error>;

    type SerializeSeq = Impossible<Self::Ok, Self::Error>;
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

impl<M: SerializeMap<Error: 'static>> SerializeStruct for RootSerializer<M> {
    type Ok = M::Ok;
    type Error = FlattenError<M::Error>;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        (&mut self.child).serialize_field(key, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.child.into_table().end()?)
    }
}

impl<M: SerializeMap<Error: 'static>> SerializeMap for RootSerializer<M> {
    type Ok = M::Ok;
    type Error = FlattenError<M::Error>;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        (&mut self.child).serialize_key(key)
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        (&mut self.child).serialize_value(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.child.into_table().end()?)
    }
}

#[derive(Debug)]
pub enum FlattenError<E> {
    Key,
    Sink(E),
}

impl<E> From<E> for FlattenError<E> {
    fn from(e: E) -> Self {
        Self::Sink(e)
    }
}

impl<E: std::fmt::Display> std::fmt::Display for FlattenError<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Key => f.write_str("invalid key"),
            Self::Sink(e) => write!(f, "{}", e),
        }
    }
}

impl<E: StdError + 'static> StdError for FlattenError<E> {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Key => None,
            Self::Sink(e) => Some(e),
        }
    }
}

impl<E: Error + 'static> Error for FlattenError<E> {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Self::Sink(E::custom(msg))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flatten_struct() {
        #[derive(Serialize)]
        struct Root {
            parent: Parent,
            child: Child,
        }

        #[derive(Serialize)]
        struct Parent {
            child: Child,
        }

        #[derive(Serialize)]
        struct Child {
            text: &'static str,
        }

        let root = Root {
            parent: Parent {
                child: Child { text: "hello" },
            },
            child: Child { text: "world" },
        };

        let mut buf = Vec::new();
        let mut json_ser = serde_json::Serializer::new(&mut buf);
        let ser = RootSerializer::new(&mut json_ser);
        root.serialize(ser).unwrap();
        assert_eq!(
            r#"{"parent.child.text":"hello","child.text":"world"}"#,
            String::from_utf8(buf).unwrap()
        );
    }

    #[test]
    #[ignore]
    fn flatten_map() {
        let root =
            serde_json::json!({"parent":{"child":{"text":"hello"}},"child":{"text":"world"}});

        let mut buf = Vec::new();
        let mut json_ser = serde_json::Serializer::new(&mut buf);
        let ser = RootSerializer::new(&mut json_ser);
        root.serialize(ser).unwrap();
        assert_eq!(
            r#"{"parent.child.text":"hello","child.text":"world"}"#,
            String::from_utf8(buf).unwrap()
        );
    }

    #[test]
    fn flatten_struct_seq() {
        #[derive(Serialize)]
        struct Root {
            parent: Vec<Parent>,
        }

        #[derive(Serialize)]
        struct Parent {
            child: Vec<Child>,
        }

        #[derive(Serialize)]
        struct Child {
            text: &'static str,
        }

        let root = Root {
            parent: vec![
                Parent {
                    child: vec![
                        Child { text: "one" },
                        Child { text: "two" },
                        Child { text: "three" },
                    ],
                },
                Parent {
                    child: vec![Child { text: "another" }],
                },
            ],
        };

        let mut buf = Vec::new();
        let mut json_ser = serde_json::Serializer::new(&mut buf);
        let ser = RootSerializer::new(&mut json_ser);
        root.serialize(ser).unwrap();
        assert_eq!(
            r#"{"parent[0].child[0].text":"one","parent[0].child[1].text":"two","parent[0].child[2].text":"three","parent[1].child[0].text":"another"}"#,
            String::from_utf8(buf).unwrap()
        );
    }

    #[test]
    fn flatten_value_seq() {
        #[derive(Serialize)]
        struct Root {
            child: Vec<Child>,
        }

        #[derive(Serialize)]
        struct Child {
            text: Vec<&'static str>,
        }

        let root = Root {
            child: vec![Child {
                text: vec!["one", "two", "three"],
            }],
        };

        let mut buf = Vec::new();
        let mut json_ser = serde_json::Serializer::new(&mut buf);
        let ser = RootSerializer::new(&mut json_ser);
        root.serialize(ser).unwrap();
        assert_eq!(
            r#"{"child[0].text":"one,two,three"}"#,
            String::from_utf8(buf).unwrap()
        );
    }
}
