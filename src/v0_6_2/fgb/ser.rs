use std::borrow::Cow;

use flatgeobuf::FgbWriter;
use geo_traits::{
    CoordTrait, GeometryCollectionTrait, GeometryTrait, GeometryType, LineStringTrait, LineTrait,
    MultiLineStringTrait, MultiPointTrait, MultiPolygonTrait, PointTrait, PolygonTrait, RectTrait,
    TriangleTrait,
};

use crate::v0_6_1::ser::prop::{
    FieldValue, FlattenOption, SerializeProperties, TableError, TableSerializer,
};

pub trait SerializeFeature {}

pub struct LayerSerializer<'a> {
    writer: FgbWriter<'a>,
    pre_orderd_key: Vec<Cow<'static, str>>,
    _flatten_opt: FlattenOption,
}

impl<'a> LayerSerializer<'a> {
    pub fn new(writer: FgbWriter<'a>) -> Self {
        LayerSerializer {
            writer,
            pre_orderd_key: Vec::new(),
            _flatten_opt: FlattenOption::full(),
        }
    }

    pub fn sort_keys(
        &mut self,
        layer: impl IntoIterator<Item = impl SerializeFeature>,
    ) -> Result<(), Error> {
        for _feat in layer.into_iter() {
            //
        }
        Ok(())
    }

    pub fn serialize_layer(
        &mut self,
        layer: impl IntoIterator<Item = impl SerializeFeature>,
    ) -> Result<(), Error> {
        for _feat in layer.into_iter() {
            //
        }
        Ok(())
    }

    pub fn into_inner(self) -> FgbWriter<'a> {
        self.writer
    }
}

/// FlatGeobuf feature serializer
pub struct FeatureSerializer<'a> {
    writer: FgbWriter<'a>,
    known_key: Vec<Cow<'static, str>>,
}

impl<'a> FeatureSerializer<'a> {
    pub fn new(writer: FgbWriter<'a>) -> Self {
        Self {
            writer,
            known_key: Vec::new(),
        }
    }

    pub fn into_inner(self) -> FgbWriter<'a> {
        self.writer
    }

    pub fn serialize_feature(
        &mut self,
        geometry: impl GeometryTrait<T = f64>,
        properties: impl serde::Serialize,
    ) -> Result<(), Error> {
        process_geometry(&geometry, &mut self.writer)?;
        let prop_ser = TableSerializer::new(&mut *self);
        properties.serialize(prop_ser)?;
        flatgeobuf::geozero::FeatureProcessor::feature_end(&mut self.writer, 0)?;
        Ok(())
    }
}

impl SerializeProperties for &mut FeatureSerializer<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_property(
        &mut self,
        key: Cow<'static, str>,
        value: FieldValue<'_>,
    ) -> Result<(), Self::Error> {
        let index_of_key = self.known_key.iter().position(|k| k == &key);
        let index_to_write = index_of_key.unwrap_or_else(|| self.known_key.len());
        flatgeobuf::geozero::PropertyProcessor::property(
            &mut self.writer,
            index_to_write,
            key.as_ref(),
            &to_column_value(value),
        )?;
        if index_of_key.is_none() {
            self.known_key.push(key);
        }
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

// --- geo_traits → geozero GeomProcessor bridge ---

fn process_geometry(
    geom: &impl GeometryTrait<T = f64>,
    processor: &mut impl flatgeobuf::geozero::GeomProcessor,
) -> Result<(), flatgeobuf::geozero::error::GeozeroError> {
    process_geometry_n(geom, 0, processor)
}

fn process_geometry_n(
    geom: &impl GeometryTrait<T = f64>,
    idx: usize,
    processor: &mut impl flatgeobuf::geozero::GeomProcessor,
) -> Result<(), flatgeobuf::geozero::error::GeozeroError> {
    match geom.as_type() {
        GeometryType::Point(g) => process_point(g, idx, processor),
        GeometryType::LineString(g) => process_line_string(g, true, idx, processor),
        GeometryType::Polygon(g) => process_polygon(g, true, idx, processor),
        GeometryType::MultiPoint(g) => process_multi_point(g, idx, processor),
        GeometryType::MultiLineString(g) => process_multi_line_string(g, idx, processor),
        GeometryType::MultiPolygon(g) => process_multi_polygon(g, idx, processor),
        GeometryType::GeometryCollection(g) => process_geometry_collection(g, idx, processor),
        GeometryType::Rect(g) => process_rect(g, idx, processor),
        GeometryType::Triangle(g) => process_triangle(g, idx, processor),
        GeometryType::Line(g) => process_line(g, idx, processor),
    }
}

fn process_coord(
    coord: &impl CoordTrait<T = f64>,
    idx: usize,
    processor: &mut impl flatgeobuf::geozero::GeomProcessor,
) -> Result<(), flatgeobuf::geozero::error::GeozeroError> {
    processor.xy(coord.x(), coord.y(), idx)
}

fn process_point(
    point: &impl PointTrait<T = f64>,
    idx: usize,
    processor: &mut impl flatgeobuf::geozero::GeomProcessor,
) -> Result<(), flatgeobuf::geozero::error::GeozeroError> {
    processor.point_begin(idx)?;
    if let Some(coord) = point.coord() {
        process_coord(&coord, 0, processor)?;
    }
    processor.point_end(idx)
}

fn process_line_string(
    ls: &impl LineStringTrait<T = f64>,
    tagged: bool,
    idx: usize,
    processor: &mut impl flatgeobuf::geozero::GeomProcessor,
) -> Result<(), flatgeobuf::geozero::error::GeozeroError> {
    processor.linestring_begin(tagged, ls.num_coords(), idx)?;
    for (i, coord) in ls.coords().enumerate() {
        process_coord(&coord, i, processor)?;
    }
    processor.linestring_end(tagged, idx)
}

fn process_polygon(
    polygon: &impl PolygonTrait<T = f64>,
    tagged: bool,
    idx: usize,
    processor: &mut impl flatgeobuf::geozero::GeomProcessor,
) -> Result<(), flatgeobuf::geozero::error::GeozeroError> {
    let num_interiors = polygon.num_interiors();
    let num_rings = if polygon.exterior().is_some() {
        num_interiors + 1
    } else {
        0
    };
    processor.polygon_begin(tagged, num_rings, idx)?;
    if let Some(exterior) = polygon.exterior() {
        process_line_string(&exterior, false, 0, processor)?;
    }
    for (i, interior) in polygon.interiors().enumerate() {
        process_line_string(&interior, false, i + 1, processor)?;
    }
    processor.polygon_end(tagged, idx)
}

fn process_multi_point(
    mp: &impl MultiPointTrait<T = f64>,
    idx: usize,
    processor: &mut impl flatgeobuf::geozero::GeomProcessor,
) -> Result<(), flatgeobuf::geozero::error::GeozeroError> {
    processor.multipoint_begin(mp.num_points(), idx)?;
    for (i, point) in mp.points().enumerate() {
        if let Some(coord) = point.coord() {
            process_coord(&coord, i, processor)?;
        }
    }
    processor.multipoint_end(idx)
}

fn process_multi_line_string(
    mls: &impl MultiLineStringTrait<T = f64>,
    idx: usize,
    processor: &mut impl flatgeobuf::geozero::GeomProcessor,
) -> Result<(), flatgeobuf::geozero::error::GeozeroError> {
    processor.multilinestring_begin(mls.num_line_strings(), idx)?;
    for (i, ls) in mls.line_strings().enumerate() {
        process_line_string(&ls, false, i, processor)?;
    }
    processor.multilinestring_end(idx)
}

fn process_multi_polygon(
    mp: &impl MultiPolygonTrait<T = f64>,
    idx: usize,
    processor: &mut impl flatgeobuf::geozero::GeomProcessor,
) -> Result<(), flatgeobuf::geozero::error::GeozeroError> {
    processor.multipolygon_begin(mp.num_polygons(), idx)?;
    for (i, polygon) in mp.polygons().enumerate() {
        process_polygon(&polygon, false, i, processor)?;
    }
    processor.multipolygon_end(idx)
}

fn process_geometry_collection(
    gc: &impl GeometryCollectionTrait<T = f64>,
    idx: usize,
    processor: &mut impl flatgeobuf::geozero::GeomProcessor,
) -> Result<(), flatgeobuf::geozero::error::GeozeroError> {
    processor.geometrycollection_begin(gc.num_geometries(), idx)?;
    for (i, geom) in gc.geometries().enumerate() {
        process_geometry_n(&geom, i, processor)?;
    }
    processor.geometrycollection_end(idx)
}

fn process_rect(
    rect: &impl RectTrait<T = f64>,
    idx: usize,
    processor: &mut impl flatgeobuf::geozero::GeomProcessor,
) -> Result<(), flatgeobuf::geozero::error::GeozeroError> {
    let min = rect.min();
    let max = rect.max();
    // Rect → Polygon with 5 coords (closed ring)
    processor.polygon_begin(true, 1, idx)?;
    processor.linestring_begin(false, 5, 0)?;
    processor.xy(min.x(), min.y(), 0)?;
    processor.xy(max.x(), min.y(), 1)?;
    processor.xy(max.x(), max.y(), 2)?;
    processor.xy(min.x(), max.y(), 3)?;
    processor.xy(min.x(), min.y(), 4)?;
    processor.linestring_end(false, 0)?;
    processor.polygon_end(true, idx)
}

fn process_triangle(
    tri: &impl TriangleTrait<T = f64>,
    idx: usize,
    processor: &mut impl flatgeobuf::geozero::GeomProcessor,
) -> Result<(), flatgeobuf::geozero::error::GeozeroError> {
    let first = tri.first();
    let second = tri.second();
    let third = tri.third();
    // Triangle → Polygon with 4 coords (closed ring)
    processor.polygon_begin(true, 1, idx)?;
    processor.linestring_begin(false, 4, 0)?;
    processor.xy(first.x(), first.y(), 0)?;
    processor.xy(second.x(), second.y(), 1)?;
    processor.xy(third.x(), third.y(), 2)?;
    processor.xy(first.x(), first.y(), 3)?;
    processor.linestring_end(false, 0)?;
    processor.polygon_end(true, idx)
}

fn process_line(
    line: &impl LineTrait<T = f64>,
    idx: usize,
    processor: &mut impl flatgeobuf::geozero::GeomProcessor,
) -> Result<(), flatgeobuf::geozero::error::GeozeroError> {
    processor.linestring_begin(true, 2, idx)?;
    let start = line.start();
    let end = line.end();
    processor.xy(start.x(), start.y(), 0)?;
    processor.xy(end.x(), end.y(), 1)?;
    processor.linestring_end(true, idx)
}

// --- FieldValue → ColumnValue ---

fn to_column_value(source: FieldValue<'_>) -> flatgeobuf::geozero::ColumnValue<'_> {
    match source {
        FieldValue::Bool(v) => flatgeobuf::geozero::ColumnValue::Bool(v),
        FieldValue::I8(v) => flatgeobuf::geozero::ColumnValue::Byte(v),
        FieldValue::I16(v) => flatgeobuf::geozero::ColumnValue::Short(v),
        FieldValue::I32(v) => flatgeobuf::geozero::ColumnValue::Int(v),
        FieldValue::I64(v) => flatgeobuf::geozero::ColumnValue::Long(v),
        FieldValue::U8(v) => flatgeobuf::geozero::ColumnValue::UByte(v),
        FieldValue::U16(v) => flatgeobuf::geozero::ColumnValue::UShort(v),
        FieldValue::U32(v) => flatgeobuf::geozero::ColumnValue::UInt(v),
        FieldValue::U64(v) => flatgeobuf::geozero::ColumnValue::ULong(v),
        FieldValue::F32(v) => flatgeobuf::geozero::ColumnValue::Float(v),
        FieldValue::F64(v) => flatgeobuf::geozero::ColumnValue::Double(v),
        FieldValue::Str(s) => flatgeobuf::geozero::ColumnValue::String(s),
        FieldValue::Bytes(b) => flatgeobuf::geozero::ColumnValue::Binary(b),
    }
}

// --- Error type ---

#[derive(Debug)]
pub enum Error {
    Geozero(flatgeobuf::geozero::error::GeozeroError),
    Source(String),
}

impl From<flatgeobuf::geozero::error::GeozeroError> for Error {
    fn from(e: flatgeobuf::geozero::error::GeozeroError) -> Self {
        Self::Geozero(e)
    }
}

impl From<TableError<Error>> for Error {
    fn from(e: TableError<Error>) -> Self {
        match e {
            TableError::Sink(e) => e,
            other => Self::Source(other.to_string()),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Geozero(_) => f.write_str("geozero processing failed"),
            Error::Source(msg) => f.write_str(msg),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Geozero(e) => Some(e),
            Error::Source(_) => None,
        }
    }
}

impl serde::ser::Error for Error {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Self::Source(msg.to_string())
    }
}
