use serde::{
    ser::{
        SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
        SerializeTupleStruct, SerializeTupleVariant,
    },
    Serialize, Serializer,
};

use super::SerializeError;

#[derive(Debug)]
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

pub trait GeometrySink {
    type Error: std::error::Error;
    fn coord(&mut self, index: usize, x: f64, y: f64) -> Result<(), Self::Error>;
    fn point_start(&mut self, index: usize) -> Result<(), Self::Error>;
    fn point_end(&mut self, index: usize) -> Result<(), Self::Error>;
    fn linestring_start(
        &mut self,
        is_child: bool,
        index: usize,
        coord_len: usize,
    ) -> Result<(), Self::Error>;
    fn linestring_end(&mut self, is_child: bool, index: usize) -> Result<(), Self::Error>;
    fn polygon_start(&mut self, is_child: bool, index: usize) -> Result<(), Self::Error>;
    fn polygon_end(&mut self, is_child: bool, index: usize) -> Result<(), Self::Error>;
    fn geometry_start(&mut self) -> Result<(), Self::Error>;
    fn geometry_end(&mut self) -> Result<(), Self::Error>;
}
#[cfg(feature = "geozero")]
impl<Z: geozero::FeatureProcessor> GeometrySink for Z {
    type Error = geozero::error::GeozeroError;
    fn coord(&mut self, index: usize, x: f64, y: f64) -> Result<(), Self::Error> {
        self.xy(x, y, index)
    }
    fn point_start(&mut self, index: usize) -> Result<(), Self::Error> {
        self.point_begin(index)
    }
    fn point_end(&mut self, index: usize) -> Result<(), Self::Error> {
        self.point_end(index)
    }
    fn linestring_start(
        &mut self,
        is_child: bool,
        index: usize,
        coord_len: usize,
    ) -> Result<(), Self::Error> {
        self.linestring_begin(!is_child, coord_len, index)
    }
    fn linestring_end(&mut self, is_child: bool, index: usize) -> Result<(), Self::Error> {
        self.linestring_end(!is_child, index)
    }
    fn polygon_start(&mut self, is_child: bool, index: usize) -> Result<(), Self::Error> {
        self.polygon_begin(!is_child, 1, index)
    }
    fn polygon_end(&mut self, is_child: bool, index: usize) -> Result<(), Self::Error> {
        self.polygon_end(!is_child, index)
    }
    fn geometry_start(&mut self) -> Result<(), Self::Error> {
        self.geometry_begin()
    }
    fn geometry_end(&mut self) -> Result<(), Self::Error> {
        self.geometry_end()
    }
}

#[derive(Debug)]
pub struct GeometrySerializer<'a, S> {
    sink: &'a mut S,
    stack: Vec<Container>,
    x: Option<f64>,
    coord_index: usize,
    point_index: usize,
    linestring_index: usize,
    polygon_index: usize,
}

impl<'a, S> GeometrySerializer<'a, S> {
    pub fn new(sink: &'a mut S) -> Self {
        Self {
            sink,
            stack: Vec::new(),
            x: None,
            coord_index: 0,
            point_index: 0,
            linestring_index: 0,
            polygon_index: 0,
        }
    }
}

impl<S: GeometrySink> Serializer for &mut GeometrySerializer<'_, S> {
    type Ok = ();
    type Error = SerializeError<S::Error>;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

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
                    if self.point_index == 0 {
                        self.sink
                            .geometry_start()
                            .map_err(SerializeError::SinkCaused)?;
                    }
                    self.sink
                        .point_start(self.point_index)
                        .map_err(SerializeError::SinkCaused)?;
                }
                [Container::Line, ..] => {
                    if self.linestring_index == 0 {
                        self.sink
                            .geometry_start()
                            .map_err(SerializeError::SinkCaused)?;
                    }
                    self.sink
                        .linestring_start(false, self.linestring_index, 2)
                        .map_err(SerializeError::SinkCaused)?;
                }
                [Container::LineString { len }, ..] => {
                    if self.linestring_index == 0 {
                        self.sink
                            .geometry_start()
                            .map_err(SerializeError::SinkCaused)?;
                    }
                    self.sink
                        .linestring_start(false, self.linestring_index, *len)
                        .map_err(SerializeError::SinkCaused)?;
                }
                [Container::Polygon, Container::LineString { len }, ..] => {
                    if self.linestring_index == 0 {
                        if self.polygon_index == 0 {
                            self.sink
                                .geometry_start()
                                .map_err(SerializeError::SinkCaused)?;
                        }
                        self.sink
                            .polygon_start(false, self.polygon_index)
                            .map_err(SerializeError::SinkCaused)?;
                    }
                    self.sink
                        .linestring_start(true, self.linestring_index, *len)
                        .map_err(SerializeError::SinkCaused)?;
                }
                [containers @ ..] => todo!("{:?}", containers),
            }
        }

        self.sink
            .coord(self.coord_index, x, v)
            .map_err(SerializeError::SinkCaused)?;
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
                self.sink
                    .point_end(self.point_index)
                    .map_err(SerializeError::SinkCaused)?;
                self.point_index += 1;
            }
            Some(Container::LineString { .. }) => {
                let is_child = self.stack.last().is_some_and(|parent| {
                    matches!(parent, Container::Polygon | Container::_MultiLineString)
                });
                self.sink
                    .linestring_end(is_child, self.linestring_index)
                    .map_err(SerializeError::SinkCaused)?;
                self.linestring_index += 1;
                self.coord_index = 0;
            }
            Some(_) => todo!(),
            None => return Err(SerializeError::InvalidState),
        }

        if self.stack.is_empty() {
            self.sink
                .geometry_end()
                .map_err(SerializeError::SinkCaused)?;
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
    type Error = SerializeError<S::Error>;

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

impl<S: GeometrySink> SerializeTuple for &mut GeometrySerializer<'_, S> {
    type Ok = ();
    type Error = SerializeError<S::Error>;
    fn serialize_element<T: ?Sized>(&mut self, _: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unimplemented!()
    }
    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}

impl<S: GeometrySink> SerializeTupleStruct for &mut GeometrySerializer<'_, S> {
    type Ok = ();
    type Error = SerializeError<S::Error>;
    fn serialize_field<T: ?Sized>(&mut self, _: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unimplemented!()
    }
    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}

impl<S: GeometrySink> SerializeTupleVariant for &mut GeometrySerializer<'_, S> {
    type Ok = ();
    type Error = SerializeError<S::Error>;

    fn serialize_field<T: ?Sized>(&mut self, _: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unimplemented!()
    }
    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}

impl<S: GeometrySink> SerializeMap for &mut GeometrySerializer<'_, S> {
    type Ok = ();
    type Error = SerializeError<S::Error>;
    fn serialize_key<T: ?Sized>(&mut self, _: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unimplemented!()
    }
    fn serialize_value<T: ?Sized>(&mut self, _: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unimplemented!()
    }
    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}

impl<S: GeometrySink> SerializeStruct for &mut GeometrySerializer<'_, S> {
    type Ok = ();
    type Error = SerializeError<S::Error>;

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
            Some(Container::Coord) => (),
            Some(Container::Line) => {
                self.sink
                    .linestring_end(false, self.linestring_index)
                    .map_err(SerializeError::SinkCaused)?;
                self.linestring_index += 1;
            }
            Some(Container::Polygon) => {
                self.sink
                    .polygon_end(false, self.polygon_index)
                    .map_err(SerializeError::SinkCaused)?;
                self.polygon_index += 1;
            }
            Some(container) => todo!("{}", container.as_str()),
            None => return Err(Self::Error::InvalidState),
        }

        if self.stack.is_empty() {
            self.sink
                .geometry_end()
                .map_err(SerializeError::SinkCaused)?;
        }
        Ok(())
    }
}

impl<S: GeometrySink> SerializeStructVariant for &mut GeometrySerializer<'_, S> {
    type Ok = ();
    type Error = SerializeError<S::Error>;
    fn serialize_field<T: ?Sized>(&mut self, _: &'static str, _: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unimplemented!()
    }
    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}
