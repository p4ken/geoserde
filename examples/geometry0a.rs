use geoserde::Feature;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Feature)]
pub struct Child1a {
    #[serde(rename = "geometry")]
    loc: geo_types::Point,
    count: i32,
}

#[derive(Serialize, Deserialize)]
pub struct MyFeature1a {
    child: Child1a,
    title: String,
}
impl MyFeature1a {
    #[rustfmt::skip]
    fn _ser(&self, serializer: impl serde::Serializer)
    {
        use serde as _serde;

        let __serializer = serializer;
        let mut __serde_state = _serde::Serializer::serialize_struct(__serializer,"MyFeature1a",false as usize+1+1).unwrap();
        _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,"child", &self.child).unwrap();
        _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,"title", &self.title).unwrap();
        _serde::ser::SerializeStruct::end(__serde_state).unwrap();
    }

    // #[rustfmt::skip]
    // fn _de<'a>(&self, deserializer: impl serde::Deserializer<'a>) {
    //     use serde as _serde;

    //     let __deserializer = deserializer;
    //     #[allow(non_camel_case_types)]
    //     #[doc(hidden)]
    //     enum __Field {
    //         __field0,__field1,__ignore,
    //     }
    //     #[doc(hidden)]
    //     struct __FieldVisitor;

    //     impl <'de>_serde::de::Visitor<'de>for __FieldVisitor {
    //         type Value = __Field;
    //         fn expecting(&self,__formatter: &mut _serde::__private::Formatter) -> _serde::__private::fmt::Result {
    //             _serde::__private::Formatter::write_str(__formatter,"field identifier")
    //         }
    //         fn visit_u64<__E>(self,__value:u64) -> _serde::__private::Result<MyFeature1a::Value,__E>where __E:_serde::de::Error,{
    //             match __value {
    //                 0u64 => _serde::__private::Ok(__Field::__field0),
    //                 1u64 => _serde::__private::Ok(__Field::__field1),
    //                 _ => _serde::__private::Ok(__Field::__ignore),

    //                 }
    //         }
    //         fn visit_str<__E>(self,__value: &str) -> _serde::__private::Result<MyFeature1a::Value,__E>where __E:_serde::de::Error,{
    //             match __value {
    //                 "child" => _serde::__private::Ok(__Field::__field0),
    //                 "title" => _serde::__private::Ok(__Field::__field1),
    //                 _ => {
    //                     _serde::__private::Ok(__Field::__ignore)
    //                 }

    //                 }
    //         }
    //         fn visit_bytes<__E>(self,__value: &[u8]) -> _serde::__private::Result<MyFeature1a::Value,__E>where __E:_serde::de::Error,{
    //             match __value {
    //                 b"child" => _serde::__private::Ok(__Field::__field0),
    //                 b"title" => _serde::__private::Ok(__Field::__field1),
    //                 _ => {
    //                     _serde::__private::Ok(__Field::__ignore)
    //                 }

    //                 }
    //         }

    //         }
    //     impl <'de>_serde::Deserialize<'de>for __Field {
    //         #[inline]
    //         fn deserialize<__D>(__deserializer:__D) -> _serde::__private::Result<MyFeature1a,__D::Error>where __D:_serde::Deserializer<'de> ,{
    //             _serde::Deserializer::deserialize_identifier(__deserializer,__FieldVisitor)
    //         }

    //         }
    //     #[doc(hidden)]
    //     struct __Visitor<'de>{
    //         marker:_serde::__private::PhantomData<MyFeature1a> ,lifetime:_serde::__private::PhantomData< &'de()> ,
    //     }
    //     impl <'de>_serde::de::Visitor<'de>for __Visitor<'de>{
    //         type Value = MyFeature1a;
    //         fn expecting(&self,__formatter: &mut _serde::__private::Formatter) -> _serde::__private::fmt::Result {
    //             _serde::__private::Formatter::write_str(__formatter,"struct MyFeature1a")
    //         }
    //         #[inline]
    //         fn visit_seq<__A>(self,mut __seq:__A) -> _serde::__private::Result<MyFeature1a::Value,__A::Error>where __A:_serde::de::SeqAccess<'de> ,{
    //             let __field0 = match _serde::de::SeqAccess::next_element:: <Child1a>(&mut __seq)?{
    //                 _serde::__private::Some(__value) => __value,
    //                 _serde::__private::None => return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"struct MyFeature1a with 2 elements")),

    //                 };
    //             let __field1 = match _serde::de::SeqAccess::next_element:: <String>(&mut __seq)?{
    //                 _serde::__private::Some(__value) => __value,
    //                 _serde::__private::None => return _serde::__private::Err(_serde::de::Error::invalid_length(1usize, &"struct MyFeature1a with 2 elements")),

    //                 };
    //             _serde::__private::Ok(MyFeature1a {
    //                 child:__field0,title:__field1
    //             })
    //         }
    //         #[inline]
    //         fn visit_map<__A>(self,mut __map:__A) -> _serde::__private::Result<MyFeature1a::Value,__A::Error>where __A:_serde::de::MapAccess<'de> ,{
    //             let mut __field0:_serde::__private::Option<Child1a>  = _serde::__private::None;
    //             let mut __field1:_serde::__private::Option<String>  = _serde::__private::None;
    //             while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key:: <__Field>(&mut __map)?{
    //                 match __key {
    //                     __Field::__field0 => {
    //                         if _serde::__private::Option::is_some(&__field0){
    //                             return _serde::__private::Err(<__A::Error as _serde::de::Error> ::duplicate_field("child"));
    //                         }__field0 = _serde::__private::Some(_serde::de::MapAccess::next_value:: <Child1a>(&mut __map)?);
    //                     }
    //                     __Field::__field1 => {
    //                         if _serde::__private::Option::is_some(&__field1){
    //                             return _serde::__private::Err(<__A::Error as _serde::de::Error> ::duplicate_field("title"));
    //                         }__field1 = _serde::__private::Some(_serde::de::MapAccess::next_value:: <String>(&mut __map)?);
    //                     }
    //                     _ => {
    //                         let _ = _serde::de::MapAccess::next_value:: <_serde::de::IgnoredAny>(&mut __map)? ;
    //                     }

    //                     }
    //             }let __field0 = match __field0 {
    //                 _serde::__private::Some(__field0) => __field0,
    //                 _serde::__private::None => _serde::__private::de::missing_field("child")? ,

    //                 };
    //             let __field1 = match __field1 {
    //                 _serde::__private::Some(__field1) => __field1,
    //                 _serde::__private::None => _serde::__private::de::missing_field("title")? ,

    //                 };
    //             _serde::__private::Ok(MyFeature1a {
    //                 child:__field0,title:__field1
    //             })
    //         }

    //         }
    //     #[doc(hidden)]
    //     const FIELDS: &'static[&'static str] =  &["child","title"];
    //     _serde::Deserializer::deserialize_struct(__deserializer,"MyFeature1a",FIELDS,__Visitor {
    //         marker:_serde::__private::PhantomData:: <MyFeature1a> ,lifetime:_serde::__private::PhantomData,
    //     }).unwrap();
    // }
}

fn main() {}
