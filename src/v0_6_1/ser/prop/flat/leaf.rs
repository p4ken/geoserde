use serde::{Serialize, Serializer, ser::Impossible};

// プリミティブ値を収集するための特殊なシリアライザ
pub struct PrimitiveCollector<E> {
    value: Option<String>,
    _phantom: std::marker::PhantomData<E>,
}

impl<E> PrimitiveCollector<E> {
    pub fn new() -> Self {
        Self {
            value: None,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn into_value(self) -> Option<String> {
        self.value
    }
}

impl<E: serde::ser::Error> Serializer for &mut PrimitiveCollector<E> {
    type Ok = ();
    type Error = E;
    type SerializeSeq = Impossible<(), Self::Error>;
    type SerializeTuple = Impossible<(), Self::Error>;
    type SerializeTupleStruct = Impossible<(), Self::Error>;
    type SerializeTupleVariant = Impossible<(), Self::Error>;
    type SerializeMap = Impossible<(), Self::Error>;
    type SerializeStruct = Impossible<(), Self::Error>;
    type SerializeStructVariant = Impossible<(), Self::Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.value = Some(v.to_string());
        Ok(())
    }
    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.value = Some(v.to_string());
        Ok(())
    }
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.value = Some(v.to_string());
        Ok(())
    }
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.value = Some(v.to_string());
        Ok(())
    }
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.value = Some(v.to_string());
        Ok(())
    }
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.value = Some(v.to_string());
        Ok(())
    }
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.value = Some(v.to_string());
        Ok(())
    }
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.value = Some(v.to_string());
        Ok(())
    }
    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.value = Some(v.to_string());
        Ok(())
    }
    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.value = Some(v.to_string());
        Ok(())
    }
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.value = Some(v.to_string());
        Ok(())
    }
    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.value = Some(v.to_string());
        Ok(())
    }
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.value = Some(v.to_string());
        Ok(())
    }
    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(E::custom("not primitive"))
    }
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(E::custom("not primitive"))
    }
    fn serialize_some<T: ?Sized + Serialize>(self, _value: &T) -> Result<Self::Ok, Self::Error> {
        Err(E::custom("not primitive"))
    }
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(E::custom("not primitive"))
    }
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(E::custom("not primitive"))
    }
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(E::custom("not primitive"))
    }
    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        Err(E::custom("not primitive"))
    }
    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        Err(E::custom("not primitive"))
    }
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(E::custom("not primitive"))
    }
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(E::custom("not primitive"))
    }
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(E::custom("not primitive"))
    }
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(E::custom("not primitive"))
    }
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(E::custom("not primitive"))
    }
    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(E::custom("not primitive"))
    }
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(E::custom("not primitive"))
    }
}
