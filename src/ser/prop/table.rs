use serde::{
    Serialize, Serializer,
    ser::{Error, Impossible, SerializeMap, SerializeStruct, StdError},
};

use crate::ser::prop::{SerializeProperties, elem::StringifyError, field::FieldSerializer};

/// Controls how nested structures and arrays are flattened into property keys.
///
/// Use [`FlattenOption::full`] for the default configuration, or chain builder
/// methods to customise separators.
///
/// # Example
///
/// ```
/// use geoserde::ser::FlattenOption;
///
/// // Use "/" instead of "." for nested attributes
/// let option = FlattenOption::full().object("/");
/// ```
#[derive(Debug)]
pub struct FlattenOption {
    /* Follow of https://gdal.org/en/stable/drivers/vector/geojson.html#open-options */
    pub(crate) flatten_nested_attribute: bool,
    pub(crate) nested_attribute_separator: &'static str,
    pub(crate) array_as_string: bool,

    /* Our original options */
    pub(crate) array_element_separator: &'static str,
    pub(crate) flatten_nested_array: bool,
    pub(crate) nested_array_index_prefix: &'static str,
    pub(crate) nested_array_index_suffix: &'static str,
}

impl FlattenOption {
    /// Returns the default option that flattens everything.
    ///
    /// Nested attributes are joined with `"."`, arrays are joined with `","`,
    /// and array indices use `"["` / `"]"` brackets.
    pub const fn full() -> Self {
        Self {
            flatten_nested_attribute: true,
            nested_attribute_separator: ".",
            array_as_string: true,
            array_element_separator: ",",
            flatten_nested_array: true,
            nested_array_index_prefix: "[",
            nested_array_index_suffix: "]",
        }
    }

    /// Enables nested-object flattening with the given key separator.
    pub const fn object(mut self, sep: &'static str) -> Self {
        self.flatten_nested_attribute = true;
        self.nested_attribute_separator = sep;
        self
    }

    /// Enables simple-array flattening, joining elements with `sep`.
    pub const fn simple_array(mut self, sep: &'static str) -> Self {
        self.array_as_string = true;
        self.array_element_separator = sep;
        self
    }

    /// Enables complex-array flattening with indexed keys.
    ///
    /// Each element is serialized under `key{prefix}{index}{suffix}`
    /// (e.g. `items[0]`).
    pub const fn object_array(mut self, prefix: &'static str, suffix: &'static str) -> Self {
        self.flatten_nested_array = true;
        self.nested_array_index_prefix = prefix;
        self.nested_array_index_suffix = suffix;
        self
    }
}

/// A serde [`Serializer`] that flattens structs and maps into key-value
/// property pairs.
///
/// The root value must be a struct or map; scalar values and sequences at
/// the root level produce a [`TableError::Root`] error.
///
/// # Example
///
/// ```
/// use geoserde::ser::{TableSerializer, SerializeProperties, FieldValue};
/// use serde::Serialize;
/// use std::borrow::Cow;
///
/// #[derive(Serialize)]
/// struct Props { name: String, value: i32 }
///
/// struct Printer;
/// impl SerializeProperties for &mut Printer {
///     type Ok = ();
///     type Error = geoserde::ser::SourceError;
///     fn serialize_property(&mut self, key: Cow<'static, str>, value: FieldValue<'_>) -> Result<(), Self::Error> { Ok(()) }
///     fn end(self) -> Result<(), Self::Error> { Ok(()) }
/// }
///
/// let mut sink = Printer;
/// let ser = TableSerializer::new(&mut sink);
/// Props { name: "a".into(), value: 1 }.serialize(ser)?;
/// # Ok::<(), geoserde::ser::TableError<geoserde::ser::SourceError>>(())
/// ```
#[derive(Debug)]
pub struct TableSerializer<P> {
    child: FieldSerializer<P>,
}

impl<P: SerializeProperties> TableSerializer<P> {
    /// Creates a new `TableSerializer` with the default [`FlattenOption::full`].
    pub fn new(sink: P) -> Self {
        Self::with_option(sink, FlattenOption::full())
    }

    /// Creates a new `TableSerializer` with custom flattening options.
    pub fn with_option(sink: P, option: FlattenOption) -> Self {
        Self {
            child: FieldSerializer::new(sink, option),
        }
    }
}

impl<P: SerializeProperties<Error: 'static>> Serializer for TableSerializer<P> {
    type Ok = P::Ok;
    type Error = TableError<P::Error>;

    type SerializeSeq = Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        Err(TableError::Root)
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        Err(TableError::Root)
    }

    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
        Err(TableError::Root)
    }

    fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
        Err(TableError::Root)
    }

    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
        Err(TableError::Root)
    }

    fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
        Err(TableError::Root)
    }

    fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
        Err(TableError::Root)
    }

    fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
        Err(TableError::Root)
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
        Err(TableError::Root)
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        Err(TableError::Root)
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        Err(TableError::Root)
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
        Err(TableError::Root)
    }

    fn serialize_str(self, _v: &str) -> Result<Self::Ok, Self::Error> {
        Err(TableError::Root)
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(TableError::Root)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(TableError::Root)
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(TableError::Root)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(TableError::Root)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(TableError::Root)
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Err(TableError::Root)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(TableError::Root)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(TableError::Root)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(TableError::Root)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(TableError::Root)
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
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(TableError::Root)
    }
}

impl<P: SerializeProperties<Error: 'static>> SerializeStruct for TableSerializer<P> {
    type Ok = P::Ok;
    type Error = TableError<P::Error>;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        (&mut self.child).serialize_field(key, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.child.into_inner().end().map_err(TableError::Sink)
    }
}

impl<P: SerializeProperties<Error: 'static>> SerializeMap for TableSerializer<P> {
    type Ok = P::Ok;
    type Error = TableError<P::Error>;

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
        self.child.into_inner().end().map_err(TableError::Sink)
    }
}

/// Error type for [`TableSerializer`].
#[derive(Debug)]
pub enum TableError<E> {
    /// The root value was not a struct or map.
    Root,
    /// A map key could not be converted to a string.
    Key(StringifyError),
    /// An error propagated from the downstream [`SerializeProperties`] sink.
    Sink(E),
}

impl<E> From<StringifyError> for TableError<E> {
    fn from(e: StringifyError) -> Self {
        Self::Key(e)
    }
}

impl<E: std::fmt::Display> std::fmt::Display for TableError<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Root => f.write_str("data source must be a map or struct"),
            Self::Key(_) => f.write_str("map key must be a string"),
            Self::Sink(_) => f.write_str("downstream serializer caused"),
        }
    }
}

impl<E: StdError + 'static> StdError for TableError<E> {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Root => None,
            Self::Key(e) => Some(e),
            Self::Sink(e) => Some(e),
        }
    }
}

impl<E: Error + 'static> Error for TableError<E> {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Self::Sink(E::custom(msg))
    }
}
