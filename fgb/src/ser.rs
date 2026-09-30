//! Writing features to FlatGeobuf.

use std::borrow::Cow;
use std::convert::Infallible;

use flatgeobuf::FgbWriter;
use geo_traits::{
    CoordTrait, GeometryCollectionTrait, GeometryTrait, GeometryType, LineStringTrait, LineTrait,
    MultiLineStringTrait, MultiPointTrait, MultiPolygonTrait, PointTrait, PolygonTrait, RectTrait,
    TriangleTrait,
};
use geoserde::ser::{FieldValue, SerializeProperties, TableError, TableSerializer};

/// Streaming FlatGeobuf feature serializer.
///
/// This serializer writes each feature to the [`FgbWriter`] as it is
/// serialized. If serializing a feature fails, nothing of that feature is
/// written, and the serializer can continue with the next feature.
///
/// # Geometry types
///
/// Whether a geometry can be written depends on the geometry type of the
/// [`FgbWriter`] header, with the default
/// [`FgbWriterOptions`](flatgeobuf::FgbWriterOptions).
///
/// | Source \ Target                                                                                                                            | [`Point`] | [`MultiPoint`] | [`LineString`] | [`MultiLineString`] | [`Polygon`] | [`MultiPolygon`] | [`GeometryCollection`] | [`Unknown`] |
/// | ------------------------------------------------------------------------------------------------------------------------------------------ | --------- | -------------- | -------------- | ------------------- | ----------- | ---------------- | ---------------------- | ----------- |
/// | [`Point`](geo_traits::GeometryType::Point)                                                                                                 | ok        | –              | –              | –                   | –           | –                | –                      | ok          |
/// | [`MultiPoint`](geo_traits::GeometryType::MultiPoint)                                                                                       | –         | ok             | –              | –                   | –           | –                | –                      | ok          |
/// | [`LineString`](geo_traits::GeometryType::LineString), [`Line`](geo_traits::GeometryType::Line)                                             | –         | –              | ok             | promoted            | –           | –                | –                      | promoted    |
/// | [`MultiLineString`](geo_traits::GeometryType::MultiLineString)                                                                             | –         | –              | –              | ok                  | –           | –                | –                      | ok          |
/// | [`Polygon`](geo_traits::GeometryType::Polygon), [`Rect`](geo_traits::GeometryType::Rect), [`Triangle`](geo_traits::GeometryType::Triangle) | –         | –              | –              | –                   | ok          | promoted         | –                      | promoted    |
/// | [`MultiPolygon`](geo_traits::GeometryType::MultiPolygon)                                                                                   | –         | –              | –              | –                   | –           | ok               | –                      | ok          |
/// | [`GeometryCollection`](geo_traits::GeometryType::GeometryCollection)                                                                       | members   | members        | members        | members             | members     | members          | –                      | members     |
///
/// [`Point`]: flatgeobuf::GeometryType::Point
/// [`MultiPoint`]: flatgeobuf::GeometryType::MultiPoint
/// [`LineString`]: flatgeobuf::GeometryType::LineString
/// [`MultiLineString`]: flatgeobuf::GeometryType::MultiLineString
/// [`Polygon`]: flatgeobuf::GeometryType::Polygon
/// [`MultiPolygon`]: flatgeobuf::GeometryType::MultiPolygon
/// [`GeometryCollection`]: flatgeobuf::GeometryType::GeometryCollection
/// [`Unknown`]: flatgeobuf::GeometryType::Unknown
///
/// - –: Fails with [`Error::Geozero`].
/// - promoted: Written as the multi type with one member, as
///   [`promote_to_multi`](flatgeobuf::FgbWriterOptions::promote_to_multi)
///   does. Fails if it is disabled.
/// - `Unknown`: The first feature fixes the geometry type of the dataset, and
///   later features follow the column of that type. With `promote_to_multi`,
///   `LineString` fixes `MultiLineString` and `Polygon` fixes `MultiPolygon`.
/// - members: [`FgbWriter`] ignores the collection itself, so its members are
///   written as one geometry of their type. See [Limitations](#limitations).
/// - `Line`, `Rect` and `Triangle` are written as `LineString` or `Polygon`.
///
/// # Limitations
///
/// - A `GeometryCollection` is not written correctly. Its members are merged
///   into one geometry, so with more than one member, the extra coordinates
///   are lost when read.
/// - Only x and y are written, and Z and M are dropped. Enabling `has_z` or
///   `has_m` in [`FgbWriterOptions`](flatgeobuf::FgbWriterOptions) is not
///   supported.
/// - Empty points in a `MultiPoint` are skipped. An empty `Point` is written
///   without coordinates.
pub struct LayerSerializer<'a> {
    writer: FgbWriter<'a>,
    known_key: Vec<Cow<'static, str>>,
    pending: Vec<(Cow<'static, str>, FieldValue<'static>)>,
}

impl<'a> LayerSerializer<'a> {
    /// Creates a new `LayerSerializer` wrapping the given [`FgbWriter`].
    pub fn new(writer: FgbWriter<'a>) -> Self {
        Self {
            writer,
            known_key: Vec::new(),
            pending: Vec::new(),
        }
    }

    /// Serializes a single feature (geometry + properties) to the writer.
    ///
    /// # Errors
    ///
    /// Returns [`Error`] if geometry processing or property serialization
    /// fails.
    pub fn serialize_feature(
        &mut self,
        geometry: impl GeometryTrait<T = f64>,
        properties: impl serde::Serialize,
    ) -> Result<(), Error> {
        // FgbWriter declares a column as soon as it receives a property, and offers no way
        // to discard a partially written feature. Buffer the fallible properties first
        // and flush them only after all of them succeed.
        // A geometry type mismatch is returned by the first *_begin, before the writer is
        // modified.
        self.pending.clear();
        let prop_ser = TableSerializer::new(&mut *self);
        properties.serialize(prop_ser)?;
        process_geometry(&geometry, &mut self.writer)?;
        for (key, value) in self.pending.drain(..) {
            let column_value = to_column_value(&value).expect("checked in serialize_property");
            let index_of_key = self.known_key.iter().position(|k| k == &key);
            let index_to_write = index_of_key.unwrap_or(self.known_key.len());
            flatgeobuf::geozero::PropertyProcessor::property(
                &mut self.writer,
                index_to_write,
                key.as_ref(),
                &column_value,
            )?;
            if index_of_key.is_none() {
                self.known_key.push(key);
            }
        }
        flatgeobuf::geozero::FeatureProcessor::feature_end(&mut self.writer, 0)?;
        Ok(())
    }

    /// Consumes this serializer and returns the inner [`FgbWriter`].
    pub fn into_inner(self) -> FgbWriter<'a> {
        self.writer
    }
}

impl SerializeProperties for &mut LayerSerializer<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_property(
        &mut self,
        key: Cow<'static, str>,
        value: FieldValue<'_>,
    ) -> Result<(), Self::Error> {
        if to_column_value(&value).is_none() {
            return Err(Error::UnsupportedFieldValue(value.into_owned()));
        }
        self.pending.push((key, value.into_owned()));
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

// --- FieldValue → flatgeobuf bridge ---

fn to_column_value<'a>(source: &'a FieldValue<'_>) -> Option<flatgeobuf::geozero::ColumnValue<'a>> {
    let column_value = match source {
        FieldValue::Bool(v) => flatgeobuf::geozero::ColumnValue::Bool(*v),
        FieldValue::I8(v) => flatgeobuf::geozero::ColumnValue::Byte(*v),
        FieldValue::I16(v) => flatgeobuf::geozero::ColumnValue::Short(*v),
        FieldValue::I32(v) => flatgeobuf::geozero::ColumnValue::Int(*v),
        FieldValue::I64(v) => flatgeobuf::geozero::ColumnValue::Long(*v),
        FieldValue::U8(v) => flatgeobuf::geozero::ColumnValue::UByte(*v),
        FieldValue::U16(v) => flatgeobuf::geozero::ColumnValue::UShort(*v),
        FieldValue::U32(v) => flatgeobuf::geozero::ColumnValue::UInt(*v),
        FieldValue::U64(v) => flatgeobuf::geozero::ColumnValue::ULong(*v),
        FieldValue::F32(v) => flatgeobuf::geozero::ColumnValue::Float(*v),
        FieldValue::F64(v) => flatgeobuf::geozero::ColumnValue::Double(*v),
        FieldValue::Str(s) => flatgeobuf::geozero::ColumnValue::String(s),
        FieldValue::Bytes(b) => flatgeobuf::geozero::ColumnValue::Binary(b),
        // In case geoserde adds a variant that has no corresponding ColumnType
        _ => return None,
    };
    Some(column_value)
}

/// Error returned when writing features to a FlatGeobuf writer.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// Error during geozero geometry or property processing.
    Geozero(flatgeobuf::geozero::error::GeozeroError),
    /// The properties could not be flattened into a table.
    Table(TableError<Infallible>),
    /// A property value has no corresponding FlatGeobuf column type.
    UnsupportedFieldValue(FieldValue<'static>),
}

impl From<flatgeobuf::geozero::error::GeozeroError> for Error {
    fn from(e: flatgeobuf::geozero::error::GeozeroError) -> Self {
        Self::Geozero(e)
    }
}

impl From<TableError<Error>> for Error {
    fn from(e: TableError<Error>) -> Self {
        match e.into_sink() {
            Ok(inner) => inner,
            Err(table) => Self::Table(table),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Geozero(_) => f.write_str("geozero processing failed"),
            Self::Table(_) => f.write_str("properties serialization failed"),
            Self::UnsupportedFieldValue(v) => write!(f, "unsupported field value: {v:?}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Geozero(e) => Some(e),
            Self::Table(e) => Some(e),
            Self::UnsupportedFieldValue(_) => None,
        }
    }
}
