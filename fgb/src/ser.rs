//! Writing features to FlatGeobuf.

use std::borrow::Cow;
use std::convert::Infallible;
use std::io::Write;

use geo_traits::{
    CoordTrait, GeometryTrait, GeometryType, LineStringTrait, LineTrait, MultiLineStringTrait,
    MultiPointTrait, MultiPolygonTrait, PointTrait, PolygonTrait, RectTrait, TriangleTrait,
};
use geoserde::ser::{FieldValue, SerializeProperties, TableError, TableSerializer};

/// FlatGeobuf feature serializer.
///
/// Features are buffered as they are serialized, and written to the output by
/// [`write`](Self::write).
///
/// # Properties
///
/// Each property is written as a column. Nested properties are flattened into
/// columns as described in [`geoserde::ser`].
///
/// # Geometry types
///
/// Whether a geometry can be written depends on the
/// [`geometry_type`](LayerOptions::geometry_type) of the layer.
///
/// | Source \ Target                                                                                                                            | [`Point`][fgb::Point] | [`MultiPoint`][fgb::MultiPoint] | [`LineString`][fgb::LineString] | [`MultiLineString`][fgb::MultiLineString] | [`Polygon`][fgb::Polygon] | [`MultiPolygon`][fgb::MultiPolygon] | [`Unknown`][fgb::Unknown] |
/// | ------------------------------------------------------------------------------------------------------------------------------------------ | --------------------- | ------------------------------- | ------------------------------- | ----------------------------------------- | ------------------------- | ----------------------------------- | ------------------------- |
/// | [`Point`](geo_traits::GeometryType::Point)                                                                                                 | ok                    | promoted                        | –                               | –                                         | –                         | –                                   | promoted                  |
/// | [`MultiPoint`](geo_traits::GeometryType::MultiPoint)                                                                                       | –                     | ok                              | –                               | –                                         | –                         | –                                   | ok                        |
/// | [`LineString`](geo_traits::GeometryType::LineString), [`Line`](geo_traits::GeometryType::Line)                                             | –                     | –                               | ok                              | promoted                                  | –                         | –                                   | promoted                  |
/// | [`MultiLineString`](geo_traits::GeometryType::MultiLineString)                                                                             | –                     | –                               | –                               | ok                                        | –                         | –                                   | ok                        |
/// | [`Polygon`](geo_traits::GeometryType::Polygon), [`Rect`](geo_traits::GeometryType::Rect), [`Triangle`](geo_traits::GeometryType::Triangle) | –                     | –                               | –                               | –                                         | ok                        | promoted                            | promoted                  |
/// | [`MultiPolygon`](geo_traits::GeometryType::MultiPolygon)                                                                                   | –                     | –                               | –                               | –                                         | –                         | ok                                  | ok                        |
///
/// [fgb::Point]: flatgeobuf::GeometryType::Point
/// [fgb::MultiPoint]: flatgeobuf::GeometryType::MultiPoint
/// [fgb::LineString]: flatgeobuf::GeometryType::LineString
/// [fgb::MultiLineString]: flatgeobuf::GeometryType::MultiLineString
/// [fgb::Polygon]: flatgeobuf::GeometryType::Polygon
/// [fgb::MultiPolygon]: flatgeobuf::GeometryType::MultiPolygon
/// [fgb::Unknown]: flatgeobuf::GeometryType::Unknown
///
/// - –: Fails with [`Error::Geozero`]. So do a `GeometryCollection` source
///   for every target, and every source for a `GeometryCollection` target.
/// - promoted: Fails by default. With
///   [`promote_to_multi`](LayerOptions::promote_to_multi), written as the
///   multi type with one member.
///
/// # Limitations
///
// TODO: ZとMも今回対応した方が良い。
/// - Only x and y are written, and Z and M are dropped.
// TODO: エラーとしてハンドリングしたい。
/// - The type of a column is fixed by the first value of its key. A later
///   value of another type, such as `i64` after `i32`, is written without an
///   error, and the file is corrupted.
pub struct LayerSerializer {
    writer: flatgeobuf::FgbWriter<'static>,
    known_key: Vec<Cow<'static, str>>,
    pending: Vec<(Cow<'static, str>, FieldValue<'static>)>,
    promote_point_to_multi: bool,
}

impl std::fmt::Debug for LayerSerializer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // `flatgeobuf::FgbWriter` does not implement `Debug`.
        f.debug_struct("LayerSerializer")
            .field("known_key", &self.known_key)
            .field("pending", &self.pending)
            .field("promote_point_to_multi", &self.promote_point_to_multi)
            .finish_non_exhaustive()
    }
}

impl LayerSerializer {
    /// Creates a new `LayerSerializer` with the default [`LayerOptions`].
    ///
    /// # Errors
    ///
    /// Returns [`Error::Fgb`] if the FlatGeobuf writer cannot be set up.
    pub fn new() -> Result<Self, Error> {
        Self::with_options(&LayerOptions::new())
    }

    /// Creates a new [`LayerOptions`] to build a `LayerSerializer` with.
    pub fn options<'a>() -> LayerOptions<'a> {
        LayerOptions::new()
    }

    fn with_options(options: &LayerOptions<'_>) -> Result<Self, Error> {
        // FgbWriter promotes LineString and Polygon only. Promote Point likewise, also in an
        // Unknown layer, which then takes the type of the promoted first feature.
        let promote_point_to_multi = options.promote_to_multi
            && matches!(
                options.geometry_type,
                flatgeobuf::GeometryType::MultiPoint | flatgeobuf::GeometryType::Unknown
            );
        let fgb_options = flatgeobuf::FgbWriterOptions {
            write_index: options.index,
            promote_to_multi: options.promote_to_multi,
            crs: flatgeobuf::FgbCrs {
                code: options.epsg,
                ..Default::default()
            },
            title: options.title,
            description: options.description,
            metadata: options.metadata,
            ..Default::default()
        };
        let writer = flatgeobuf::FgbWriter::create_with_options(
            options.name,
            options.geometry_type,
            fgb_options,
        )?;
        Ok(Self {
            writer,
            known_key: Vec::new(),
            pending: Vec::new(),
            promote_point_to_multi,
        })
    }

    /// Serializes a feature from a geometry and a properties struct.
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
        match geometry.as_type() {
            GeometryType::Point(p) if self.promote_point_to_multi => {
                process_point_as_multi(p, &mut self.writer)?
            }
            _ => process_geometry(&geometry, &mut self.writer)?,
        }
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

    /// Writes the layer to `out`, and flushes it.
    ///
    /// Nothing is written until this is called. The layer is written in many
    /// small writes, so wrap a [`File`](std::fs::File) in a
    /// [`BufWriter`](std::io::BufWriter).
    ///
    /// # Errors
    ///
    /// Returns [`Error::Fgb`] if writing to `out` fails.
    pub fn write(self, mut out: impl Write) -> Result<(), Error> {
        self.writer.write(&mut out)?;
        out.flush().map_err(flatgeobuf::Error::IO)?;
        Ok(())
    }

    /// Unwraps this `LayerSerializer`, returning the underlying writer.
    ///
    /// This is for advanced use cases that `LayerSerializer` does not
    /// offer. To just write the layer, use [`write`](Self::write) instead.
    pub fn into_inner(self) -> flatgeobuf::FgbWriter<'static> {
        self.writer
    }
}

/// Builder of [`LayerSerializer`], with the options for writing a FlatGeobuf
/// layer.
///
/// # Example
///
/// ```
/// # fn main() -> Result<(), geoserde_fgb::ser::Error> {
/// let ser = geoserde_fgb::LayerSerializer::options()
///     .geometry_type(geoserde_fgb::flatgeobuf::GeometryType::Point)
///     .epsg(4326)
///     .create()?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct LayerOptions<'a> {
    name: &'a str,
    geometry_type: flatgeobuf::GeometryType,
    epsg: i32,
    index: bool,
    promote_to_multi: bool,
    title: Option<&'a str>,
    description: Option<&'a str>,
    metadata: Option<&'a str>,
}

impl Default for LayerOptions<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> LayerOptions<'a> {
    /// Creates a new `LayerOptions` with the default options.
    pub fn new() -> Self {
        Self {
            name: "",
            geometry_type: flatgeobuf::GeometryType::Unknown,
            epsg: 0,
            index: true,
            // Unlike flatgeobuf crate, following GDAL (PROMOTE_TO_MULTI).
            promote_to_multi: false,
            title: None,
            description: None,
            metadata: None,
        }
    }

    /// Sets the layer name. Defaults to empty.
    pub fn name(&mut self, name: &'a str) -> &mut Self {
        self.name = name;
        self
    }

    /// Sets the geometry type of the layer. Defaults to
    /// [`Unknown`](flatgeobuf::GeometryType::Unknown).
    ///
    /// See [Geometry types](LayerSerializer#geometry-types).
    pub fn geometry_type(&mut self, geometry_type: flatgeobuf::GeometryType) -> &mut Self {
        self.geometry_type = geometry_type;
        self
    }

    /// Sets the EPSG code of the coordinate reference system. Defaults to
    /// unknown.
    pub fn epsg(&mut self, code: i32) -> &mut Self {
        self.epsg = code;
        self
    }

    /// Sets whether to write the spatial index. Defaults to `true`.
    ///
    /// With the index, features are sorted along a Hilbert curve, so they are
    /// not read in the order they were written.
    pub fn index(&mut self, index: bool) -> &mut Self {
        self.index = index;
        self
    }

    /// Sets whether to write a single geometry as the multi type with one
    /// member. Defaults to `false`, unlike [`flatgeobuf::FgbWriterOptions`].
    ///
    /// See [Geometry types](LayerSerializer#geometry-types).
    pub fn promote_to_multi(&mut self, promote_to_multi: bool) -> &mut Self {
        self.promote_to_multi = promote_to_multi;
        self
    }

    /// Sets the layer title.
    pub fn title(&mut self, title: &'a str) -> &mut Self {
        self.title = Some(title);
        self
    }

    /// Sets the layer description.
    pub fn description(&mut self, description: &'a str) -> &mut Self {
        self.description = Some(description);
        self
    }

    /// Sets the application specific metadata of the layer.
    pub fn metadata(&mut self, metadata: &'a str) -> &mut Self {
        self.metadata = Some(metadata);
        self
    }

    /// Creates a new `LayerSerializer` with these options.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Fgb`] if the FlatGeobuf writer cannot be set up.
    pub fn create(&self) -> Result<LayerSerializer, Error> {
        LayerSerializer::with_options(self)
    }
}

impl SerializeProperties for &mut LayerSerializer {
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
    match geom.as_type() {
        GeometryType::Point(g) => process_point(g, processor),
        GeometryType::LineString(g) => process_line_string(g, true, 0, processor),
        GeometryType::Polygon(g) => process_polygon(g, true, 0, processor),
        GeometryType::MultiPoint(g) => process_multi_point(g, processor),
        GeometryType::MultiLineString(g) => process_multi_line_string(g, processor),
        GeometryType::MultiPolygon(g) => process_multi_polygon(g, processor),
        // FgbWriter (flatgeobuf 6.0.1) drops geometrycollection_* calls, so the members would
        // be merged into one geometry, and members of mixed types corrupt the next feature.
        GeometryType::GeometryCollection(_) => {
            Err(flatgeobuf::geozero::error::GeozeroError::Geometry(
                "GeometryCollection is not supported".to_string(),
            ))
        }
        GeometryType::Rect(g) => process_rect(g, processor),
        GeometryType::Triangle(g) => process_triangle(g, processor),
        GeometryType::Line(g) => process_line(g, processor),
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
    processor: &mut impl flatgeobuf::geozero::GeomProcessor,
) -> Result<(), flatgeobuf::geozero::error::GeozeroError> {
    let Some(coord) = point.coord() else {
        return processor.empty_point(0);
    };
    processor.point_begin(0)?;
    process_coord(&coord, 0, processor)?;
    processor.point_end(0)
}

fn process_point_as_multi(
    point: &impl PointTrait<T = f64>,
    processor: &mut impl flatgeobuf::geozero::GeomProcessor,
) -> Result<(), flatgeobuf::geozero::error::GeozeroError> {
    let Some(coord) = point.coord() else {
        return processor.empty_point(0);
    };
    processor.multipoint_begin(1, 0)?;
    process_coord(&coord, 0, processor)?;
    processor.multipoint_end(0)
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
    processor: &mut impl flatgeobuf::geozero::GeomProcessor,
) -> Result<(), flatgeobuf::geozero::error::GeozeroError> {
    processor.multipoint_begin(mp.num_points(), 0)?;
    for (i, point) in mp.points().enumerate() {
        // Skipped, as geozero has no way to pass an empty member.
        if let Some(coord) = point.coord() {
            process_coord(&coord, i, processor)?;
        }
    }
    processor.multipoint_end(0)
}

fn process_multi_line_string(
    mls: &impl MultiLineStringTrait<T = f64>,
    processor: &mut impl flatgeobuf::geozero::GeomProcessor,
) -> Result<(), flatgeobuf::geozero::error::GeozeroError> {
    processor.multilinestring_begin(mls.num_line_strings(), 0)?;
    for (i, ls) in mls.line_strings().enumerate() {
        process_line_string(&ls, false, i, processor)?;
    }
    processor.multilinestring_end(0)
}

fn process_multi_polygon(
    mp: &impl MultiPolygonTrait<T = f64>,
    processor: &mut impl flatgeobuf::geozero::GeomProcessor,
) -> Result<(), flatgeobuf::geozero::error::GeozeroError> {
    processor.multipolygon_begin(mp.num_polygons(), 0)?;
    for (i, polygon) in mp.polygons().enumerate() {
        process_polygon(&polygon, false, i, processor)?;
    }
    processor.multipolygon_end(0)
}

fn process_rect(
    rect: &impl RectTrait<T = f64>,
    processor: &mut impl flatgeobuf::geozero::GeomProcessor,
) -> Result<(), flatgeobuf::geozero::error::GeozeroError> {
    let min = rect.min();
    let max = rect.max();
    // Rect → Polygon with 5 coords (closed ring)
    processor.polygon_begin(true, 1, 0)?;
    processor.linestring_begin(false, 5, 0)?;
    processor.xy(min.x(), min.y(), 0)?;
    processor.xy(max.x(), min.y(), 1)?;
    processor.xy(max.x(), max.y(), 2)?;
    processor.xy(min.x(), max.y(), 3)?;
    processor.xy(min.x(), min.y(), 4)?;
    processor.linestring_end(false, 0)?;
    processor.polygon_end(true, 0)
}

fn process_triangle(
    tri: &impl TriangleTrait<T = f64>,
    processor: &mut impl flatgeobuf::geozero::GeomProcessor,
) -> Result<(), flatgeobuf::geozero::error::GeozeroError> {
    let first = tri.first();
    let second = tri.second();
    let third = tri.third();
    // Triangle → Polygon with 4 coords (closed ring)
    processor.polygon_begin(true, 1, 0)?;
    processor.linestring_begin(false, 4, 0)?;
    processor.xy(first.x(), first.y(), 0)?;
    processor.xy(second.x(), second.y(), 1)?;
    processor.xy(third.x(), third.y(), 2)?;
    processor.xy(first.x(), first.y(), 3)?;
    processor.linestring_end(false, 0)?;
    processor.polygon_end(true, 0)
}

fn process_line(
    line: &impl LineTrait<T = f64>,
    processor: &mut impl flatgeobuf::geozero::GeomProcessor,
) -> Result<(), flatgeobuf::geozero::error::GeozeroError> {
    processor.linestring_begin(true, 2, 0)?;
    let start = line.start();
    let end = line.end();
    processor.xy(start.x(), start.y(), 0)?;
    processor.xy(end.x(), end.y(), 1)?;
    processor.linestring_end(true, 0)
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
    /// FlatGeobuf error while setting up the writer or writing the output.
    Fgb(flatgeobuf::Error),
    /// Error during geozero geometry or property processing.
    Geozero(flatgeobuf::geozero::error::GeozeroError),
    /// The properties could not be flattened into a table.
    Table(TableError<Infallible>),
    /// A property value has no corresponding FlatGeobuf column type.
    UnsupportedFieldValue(FieldValue<'static>),
}

impl From<flatgeobuf::Error> for Error {
    fn from(e: flatgeobuf::Error) -> Self {
        Self::Fgb(e)
    }
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
            Self::Fgb(_) => f.write_str("flatgeobuf writing failed"),
            Self::Geozero(_) => f.write_str("geozero processing failed"),
            Self::Table(_) => f.write_str("properties serialization failed"),
            Self::UnsupportedFieldValue(v) => write!(f, "unsupported field value: {v:?}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Fgb(e) => Some(e),
            Self::Geozero(e) => Some(e),
            Self::Table(e) => Some(e),
            Self::UnsupportedFieldValue(_) => None,
        }
    }
}
