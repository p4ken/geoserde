use serde::{
    ser::{Impossible, SerializeSeq, SerializeStruct},
    Serialize, Serializer,
};

use crate::{GeometrySink, SerializeError};

#[derive(Debug, Clone, Copy)]
enum Container {
    Coord,
    Point,
    _MultiPoint,
    Line,
    LineString { len: usize },
    _MultiLineString,
    Polygon,
    _MultiPolygon,
    _Geometry,
    _GeometryCollection,
}
impl Container {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Coord => "Coord",
            Self::Point => "Point",
            Self::_MultiPoint => "MultiPoint",
            Self::Line => "Line",
            Self::LineString { .. } => "LineString",
            Self::_MultiLineString => "MultiLineString",
            Self::Polygon => "Polygon",
            Self::_MultiPolygon => "MultiPolygon",
            Self::_Geometry => "Geometry",
            Self::_GeometryCollection => "GeometryCollection",
        }
    }
}

#[derive(Debug)]
struct Ribbon<T> {
    inner: T,
    used: bool,
}
impl<T> Ribbon<T> {
    fn new(inner: T) -> Self {
        Self { inner, used: false }
    }
    fn get(&mut self) -> &mut T {
        self.used = true;
        &mut self.inner
    }
    fn is_used(&self) -> bool {
        self.used
    }
}

/// Serialize geometries to GIS formats.
///
/// Currently the following types can be serialized:
/// - [`geo_types::Point`]
/// - [`geo_types::Line`]
/// - [`geo_types::LineString`]
/// - [`geo_types::Polygon`]
///
/// Any other type will result in an error.
#[derive(Debug)]
pub struct GeometrySerializer<'a, S> {
    sink: Ribbon<&'a mut S>,
    stack: Vec<Container>,
    x: Option<f64>,
    coord_index: usize,
    point_index: usize,
    linestring_index: usize,
    polygon_index: usize,
}

impl<'a, S: GeometrySink> GeometrySerializer<'a, S> {
    /// Create a new `GeometrySerializer` with a [`GeometrySink`].
    ///
    /// # Examples
    ///
    /// ```
    /// let mut sink = geozero::ProcessorSink;
    /// let mut ser = geoserde::PropertySerializer::new(0, "", &mut sink);
    /// ```
    pub fn new(sink: &'a mut S) -> Self {
        Self {
            sink: Ribbon::new(sink),
            stack: Vec::new(),
            x: None,
            coord_index: 0,
            point_index: 0,
            linestring_index: 0,
            polygon_index: 0,
        }
    }

    /// Whether something has been written to the sink.
    ///
    /// # Examples
    ///
    /// ```
    /// # use serde::ser::Serialize;
    ///
    /// let mut sink = geozero::ProcessorSink;
    /// let mut ser = geoserde::GeometrySerializer::new(&mut sink);
    /// assert!(!ser.is_sink_used());
    ///
    /// geo_types::Point::new(51.5321, -0.1233).serialize(&mut ser);
    /// assert!(ser.is_sink_used());
    /// ```
    pub fn is_sink_used(&self) -> bool {
        self.sink.is_used()
    }

    fn write_coord(&mut self, x: f64, y: f64) -> Result<(), SerializeError<S::Err>> {
        self.sink
            .get()
            .coord(self.coord_index, x, y)
            .map_err(SerializeError::SinkCaused)
    }

    fn start_geometry(&mut self) -> Result<(), SerializeError<S::Err>> {
        self.sink
            .get()
            .geometry_start()
            .map_err(SerializeError::SinkCaused)
    }

    fn end_geometry(&mut self) -> Result<(), SerializeError<S::Err>> {
        self.sink
            .get()
            .geometry_end()
            .map_err(SerializeError::SinkCaused)
    }

    fn start_point_geometry(&mut self) -> Result<(), SerializeError<S::Err>> {
        if self.point_index == 0 {
            self.start_geometry()?;
        }
        self.sink
            .get()
            .point_start(self.point_index)
            .map_err(SerializeError::SinkCaused)
    }

    fn end_point_geometry(&mut self) -> Result<(), SerializeError<S::Err>> {
        self.sink
            .get()
            .point_end(self.point_index)
            .map_err(SerializeError::SinkCaused)?;
        self.point_index += 1;
        Ok(())
    }

    fn start_linestring_geometry(
        &mut self,
        coord_len: usize,
    ) -> Result<(), SerializeError<S::Err>> {
        if self.linestring_index == 0 {
            self.start_geometry()?;
        }
        self.sink
            .get()
            .linestring_start(false, self.linestring_index, coord_len)
            .map_err(SerializeError::SinkCaused)
    }

    fn end_linestring_geometry(&mut self) -> Result<(), SerializeError<S::Err>> {
        self.sink
            .get()
            .linestring_end(false, self.linestring_index)
            .map_err(SerializeError::SinkCaused)?;
        self.linestring_index += 1;
        self.coord_index = 0;
        Ok(())
    }

    fn start_polygon_geometry(&mut self) -> Result<(), SerializeError<S::Err>> {
        if self.polygon_index == 0 {
            self.start_geometry()?;
        }
        self.sink
            .get()
            .polygon_start(false, self.polygon_index)
            .map_err(SerializeError::SinkCaused)
    }

    fn end_polygon_geometry(&mut self) -> Result<(), SerializeError<S::Err>> {
        self.sink
            .get()
            .polygon_end(false, self.polygon_index)
            .map_err(SerializeError::SinkCaused)?;
        self.polygon_index += 1;
        Ok(())
    }

    fn start_polygon_linestring(&mut self, coord_len: usize) -> Result<(), SerializeError<S::Err>> {
        if self.linestring_index == 0 {
            self.start_polygon_geometry()?;
        }
        self.sink
            .get()
            .linestring_start(true, self.linestring_index, coord_len)
            .map_err(SerializeError::SinkCaused)
    }

    fn end_polygon_linestring(&mut self) -> Result<(), SerializeError<S::Err>> {
        self.sink
            .get()
            .linestring_end(true, self.linestring_index)
            .map_err(SerializeError::SinkCaused)?;
        self.linestring_index += 1;
        self.coord_index = 0;
        Ok(())
    }
}

impl<S: GeometrySink> Serializer for &mut GeometrySerializer<'_, S> {
    type Ok = ();
    type Error = SerializeError<S::Err>;
    type SerializeSeq = Self;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = Self;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::InvalidGeometryStructure {
            expected: None,
            actual: "bool",
        })
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(v.into())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(v.into())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(v.into())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(v as f64)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(v.into())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(v.into())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(v.into())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(v as f64)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(v.into())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        match self.stack.last() {
            Some(Container::Coord) => (),
            arm => {
                return Err(SerializeError::InvalidGeometryStructure {
                    expected: Some("Coord"),
                    actual: match arm {
                        Some(container) => container.as_str(),
                        None => "None",
                    },
                })
            }
        }

        let x = match self.x {
            Some(x) => x,
            None => return Ok(self.x = Some(v)),
        };

        if self.coord_index == 0 {
            match &self.stack[..] {
                [Container::Point, ..] => {
                    self.start_point_geometry()?;
                }
                [Container::Line, ..] => {
                    self.start_linestring_geometry(2)?;
                }
                [Container::LineString { len }, ..] => {
                    self.start_linestring_geometry(*len)?;
                }
                [Container::Polygon, Container::LineString { len }, ..] => {
                    self.start_polygon_linestring(*len)?;
                }
                [containers @ ..] => todo!("{:?}", containers),
            }
        }

        self.write_coord(x, v)?;
        self.x = None;
        self.coord_index += 1;

        Ok(())
    }

    fn serialize_char(self, _: char) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::InvalidGeometryStructure {
            expected: None,
            actual: "char",
        })
    }

    fn serialize_str(self, _: &str) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::InvalidGeometryStructure {
            expected: None,
            actual: "str",
        })
    }

    fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::InvalidGeometryStructure {
            expected: None,
            actual: "bytes",
        })
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::InvalidGeometryStructure {
            expected: None,
            actual: "None",
        })
    }

    fn serialize_some<T: ?Sized>(self, _: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(SerializeError::InvalidGeometryStructure {
            expected: None,
            actual: "Some",
        })
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::InvalidGeometryStructure {
            expected: None,
            actual: "unit",
        })
    }

    fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::InvalidGeometryStructure {
            expected: None,
            actual: "unit struct",
        })
    }

    fn serialize_unit_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::InvalidGeometryStructure {
            expected: None,
            actual: "unit variant",
        })
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        // dbg!(name);

        let container = match name {
            "LineString" => Container::LineString { len: 0 },
            "Point" => Container::Point,
            name => {
                return Err(SerializeError::InvalidGeometryStructure {
                    expected: Some("geometry type"),
                    actual: name,
                })
            }
        };
        self.stack.push(container);

        value.serialize(&mut *self)?;

        match self.stack.pop() {
            Some(Container::Point) => {
                self.end_point_geometry()?;
            }
            Some(Container::LineString { .. }) => match self.stack.last() {
                None => self.end_linestring_geometry()?,
                Some(Container::Polygon) => self.end_polygon_linestring()?,
                // Some(Container::_MultiLineString) => self.end_multi_linestring()?,
                Some(container) => todo!("{:?}", container),
            },
            Some(_) => todo!(),
            None => return Err(SerializeError::InvalidState),
        }

        if self.stack.is_empty() {
            self.end_geometry()?;
        }

        // dbg!();
        Ok(())
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        // TODO Geometry
        Err(SerializeError::InvalidGeometryStructure {
            expected: Some("Geometry variant"),
            actual: name,
        })
    }

    fn serialize_seq(self, seq_len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        match self.stack.last_mut() {
            Some(Container::LineString { len }) => {
                *len = seq_len.ok_or(SerializeError::InvalidGeometryStructure {
                    expected: Some("known length seq"),
                    actual: "unknown length",
                })?;
            }
            Some(Container::Polygon) => (),
            Some(container) => todo!("{}", container.as_str()),
            None => {
                return Err(SerializeError::InvalidGeometryStructure {
                    expected: Some("sequene in container"),
                    actual: "raw sequence",
                })
            }
        }
        Ok(self)
    }

    fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(SerializeError::InvalidGeometryStructure {
            expected: None,
            actual: "tuple",
        })
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        // TODO Triangle
        Err(SerializeError::InvalidGeometryStructure {
            expected: None,
            actual: "tuple struct",
        })
    }

    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(SerializeError::InvalidGeometryStructure {
            expected: None,
            actual: "tuple variant",
        })
    }

    fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(SerializeError::InvalidGeometryStructure {
            expected: None,
            actual: "map",
        })
    }

    fn serialize_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        // dbg!(name);
        let container = match name {
            "Coord" => Container::Coord,
            "Line" => Container::Line,
            "Polygon" => Container::Polygon,
            name => {
                return Err(SerializeError::InvalidGeometryStructure {
                    expected: None,
                    actual: name,
                })
            }
        };
        self.stack.push(container);
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(SerializeError::InvalidGeometryStructure {
            expected: None,
            actual: "struct variant",
        })
    }
}

impl<S: GeometrySink> SerializeSeq for &mut GeometrySerializer<'_, S> {
    type Ok = ();
    type Error = SerializeError<S::Err>;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        // dbg!();
        Ok(())
    }
}

impl<S: GeometrySink> SerializeStruct for &mut GeometrySerializer<'_, S> {
    type Ok = ();
    type Error = SerializeError<S::Err>;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        // dbg!(_key);
        value.serialize(&mut **self)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        match self.stack.pop() {
            Some(Container::Coord) => {
                if self.coord_index == 0 {
                    return Err(Self::Error::InvalidGeometryStructure {
                        expected: Some("x y"),
                        actual: "Coord end",
                    });
                }
            }
            Some(Container::Line) => {
                if self.coord_index != 2 {
                    return Err(SerializeError::InvalidGeometryStructure {
                        expected: Some("2 coords"),
                        actual: "Line end",
                    });
                }
                self.end_linestring_geometry()?;
            }
            Some(Container::Polygon) => {
                if self.linestring_index == 0 {
                    return Err(Self::Error::InvalidGeometryStructure {
                        expected: Some("LineString"),
                        actual: "Polygon end",
                    });
                }
                self.end_polygon_geometry()?;
            }
            Some(container) => todo!("{}", container.as_str()),
            None => return Err(Self::Error::InvalidState),
        }

        if self.stack.is_empty() {
            self.end_geometry()?;
        }
        Ok(())
    }
}
