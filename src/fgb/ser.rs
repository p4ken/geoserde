use std::borrow::Cow;

use flatgeobuf::FgbWriter;
use geo_traits::{
    CoordTrait, GeometryCollectionTrait, GeometryTrait, GeometryType, LineStringTrait, LineTrait,
    MultiLineStringTrait, MultiPointTrait, MultiPolygonTrait, PointTrait, PolygonTrait, RectTrait,
    TriangleTrait,
    to_geo::{
        ToGeoLine, ToGeoLineString, ToGeoMultiLineString, ToGeoMultiPoint, ToGeoMultiPolygon,
        ToGeoPoint, ToGeoPolygon, ToGeoRect, ToGeoTriangle,
    },
};

use crate::fgb::Error;
use crate::ser::{FieldValue, FlatProperties, SerializeProperties, TableSerializer};

/// FlatGeobuf layer serializer that collects features in memory before writing.
///
/// This serializer works in two phases:
///
/// 1. Call [`add_feature`](Self::add_feature) for each feature to flatten its
///    properties and store them alongside the geometry.
/// 2. Optionally reorder columns with [`set_columns`](Self::set_columns),
///    then call [`write_features`](Self::write_features) with an
///    [`FgbWriter`] to emit all features at once.
///
/// This two-pass approach allows the column set and order to be determined
/// from the union of all features before any data is written.
#[derive(Debug)]
pub struct LayerSerializer {
    column_idx: std::collections::HashMap<String, u32>,
    columns: Vec<String>,
    column_types: Vec<flatgeobuf::ColumnType>,
    geometries: Vec<geo_types::Geometry<f64>>,
    /// Flat pool of all features' entries, concatenated.
    prop_pool: Vec<(u32, FieldValue<'static>)>,
    /// Cumulative end-offsets into `prop_pool` for each feature.
    prop_offsets: Vec<u32>,
}

impl LayerSerializer {
    /// Creates a new, empty `LayerSerializer`.
    pub fn new() -> Self {
        LayerSerializer {
            column_idx: std::collections::HashMap::new(),
            columns: Vec::new(),
            column_types: Vec::new(),
            geometries: Vec::new(),
            prop_pool: Vec::new(),
            prop_offsets: vec![0],
        }
    }

    /// Adds a feature (geometry + properties) to this layer.
    ///
    /// # Errors
    ///
    /// Returns [`Error`] if the properties cannot be flattened.
    pub fn add_feature(
        &mut self,
        geometry: impl GeometryTrait<T = f64>,
        properties: impl serde::Serialize,
    ) -> Result<(), Error> {
        let entries = FlatProperties::flatten(properties)?.into_entries();
        for (key, value) in entries {
            let idx = self
                .column_idx
                .get(key.as_ref())
                .copied()
                .unwrap_or_else(|| {
                    let i = self.columns.len() as u32;
                    let owned = key.into_owned();
                    self.columns.push(owned.clone());
                    self.column_types.push(to_column_type(&value));
                    self.column_idx.insert(owned, i);
                    i
                });
            self.prop_pool.push((idx, value));
        }
        self.geometries.push(to_geo_geometry(&geometry));
        self.prop_offsets.push(self.prop_pool.len() as u32);
        Ok(())
    }

    /// Returns an iterator over the union of all property keys seen so far.
    pub fn keys(&self) -> impl Iterator<Item = &str> {
        self.columns.iter().map(|s| s.as_str())
    }

    /// Sets the column order used when writing features.
    ///
    /// `columns` must contain exactly the same set of columns as the
    /// current layer (reordering only; adding or removing columns is
    /// not supported).
    ///
    /// # Errors
    ///
    /// Returns [`Error`] if `columns` does not match the existing column set.
    pub fn set_columns(&mut self, columns: Vec<String>) -> Result<(), Error> {
        let new_idx: std::collections::HashMap<String, u32> =
            columns.iter().cloned().zip(0u32..).collect();
        let remap: Vec<u32> = self
            .columns
            .iter()
            .map(|k| new_idx.get(k).copied())
            .collect::<Option<_>>()
            .ok_or_else(|| {
                Error::Source(format!(
                    "column set mismatch: expected {:?}, got {:?}",
                    self.columns, columns,
                ))
            })?;
        let mut new_types = vec![flatgeobuf::ColumnType::String; columns.len()];
        for (old, &ty) in self.column_types.iter().enumerate() {
            new_types[remap[old] as usize] = ty;
        }
        for (idx, _) in &mut self.prop_pool {
            *idx = remap[*idx as usize];
        }
        self.columns = columns;
        self.column_idx = new_idx;
        self.column_types = new_types;
        Ok(())
    }

    /// Writes all collected features to the given [`FgbWriter`].
    ///
    /// Columns are declared in the current key order before any features
    /// are written. Memory is released incrementally as features are
    /// consumed.
    ///
    /// # Errors
    ///
    /// Returns [`Error`] if geometry processing or property writing fails.
    pub fn write_features(self, writer: &mut FgbWriter<'_>) -> Result<(), Error> {
        let LayerSerializer {
            column_idx: _,
            columns,
            column_types,
            mut geometries,
            mut prop_pool,
            prop_offsets,
        } = self;

        for (key, ty) in columns.iter().zip(column_types.into_iter()) {
            writer.add_column(key, ty, |_, _| {});
        }

        // Reverse so that pop() yields features in the original order.
        geometries.reverse();
        prop_pool.reverse();

        let n_features = prop_offsets.len() - 1;
        for i in 0..n_features {
            let geom = geometries.pop().expect("geometries/prop_offsets mismatch");
            process_geometry(&geom, writer)?;
            drop(geom);

            let count = (prop_offsets[i + 1] - prop_offsets[i]) as usize;
            for _ in 0..count {
                let (idx, val) = prop_pool.pop().expect("prop_pool/offsets mismatch");
                flatgeobuf::geozero::PropertyProcessor::property(
                    writer,
                    idx as usize,
                    columns[idx as usize].as_ref(),
                    &to_column_value(&val),
                )?;
            }
            flatgeobuf::geozero::FeatureProcessor::feature_end(writer, 0)?;

            // Periodically shrink backing buffers to return memory to the OS.
            if (i + 1) % 1024 == 0 {
                geometries.shrink_to_fit();
                prop_pool.shrink_to_fit();
            }
        }
        Ok(())
    }
}

/// Streaming FlatGeobuf feature serializer.
///
/// Unlike [`LayerSerializer`], this serializer writes each feature directly
/// to the [`FgbWriter`] as it is serialized, without buffering.
pub struct FeatureSerializer<'a> {
    writer: FgbWriter<'a>,
    known_key: Vec<Cow<'static, str>>,
}

impl<'a> FeatureSerializer<'a> {
    /// Creates a new `FeatureSerializer` wrapping the given [`FgbWriter`].
    pub fn new(writer: FgbWriter<'a>) -> Self {
        Self {
            writer,
            known_key: Vec::new(),
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
        process_geometry(&geometry, &mut self.writer)?;
        let prop_ser = TableSerializer::new(&mut *self);
        properties.serialize(prop_ser)?;
        flatgeobuf::geozero::FeatureProcessor::feature_end(&mut self.writer, 0)?;
        Ok(())
    }

    /// Consumes this serializer and returns the inner [`FgbWriter`].
    pub fn into_inner(self) -> FgbWriter<'a> {
        self.writer
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
            &to_column_value(&value),
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

// --- GeometryTrait → geo_types bridge ---
//
// `ToGeoGeometry` の blanket impl は trait solver overflow (rustc #128887) を
// 起こすため、`as_type()` で分解して個別の `ToGeoXxx` で変換する。
// `GeometryCollection` は `ToGeoGeometryCollection` も内部で `ToGeoGeometry` を
// 呼ぶため手動で再帰する。

fn to_geo_geometry(geom: &impl GeometryTrait<T = f64>) -> geo_types::Geometry<f64> {
    match geom.as_type() {
        GeometryType::Point(g) => geo_types::Geometry::Point(g.to_point()),
        GeometryType::LineString(g) => geo_types::Geometry::LineString(g.to_line_string()),
        GeometryType::Polygon(g) => geo_types::Geometry::Polygon(g.to_polygon()),
        GeometryType::MultiPoint(g) => geo_types::Geometry::MultiPoint(g.to_multi_point()),
        GeometryType::MultiLineString(g) => {
            geo_types::Geometry::MultiLineString(g.to_multi_line_string())
        }
        GeometryType::MultiPolygon(g) => geo_types::Geometry::MultiPolygon(g.to_multi_polygon()),
        GeometryType::GeometryCollection(g) => {
            let geoms = g.geometries().map(|g| to_geo_geometry(&g)).collect();
            geo_types::Geometry::GeometryCollection(geo_types::GeometryCollection(geoms))
        }
        GeometryType::Rect(g) => geo_types::Geometry::Rect(g.to_rect()),
        GeometryType::Triangle(g) => geo_types::Geometry::Triangle(g.to_triangle()),
        GeometryType::Line(g) => geo_types::Geometry::Line(g.to_line()),
    }
}

// --- FieldValue → flatgeobuf bridge ---

fn to_column_type(source: &FieldValue<'_>) -> flatgeobuf::ColumnType {
    match source {
        FieldValue::Bool(_) => flatgeobuf::ColumnType::Bool,
        FieldValue::I8(_) => flatgeobuf::ColumnType::Byte,
        FieldValue::I16(_) => flatgeobuf::ColumnType::Short,
        FieldValue::I32(_) => flatgeobuf::ColumnType::Int,
        FieldValue::I64(_) => flatgeobuf::ColumnType::Long,
        FieldValue::U8(_) => flatgeobuf::ColumnType::UByte,
        FieldValue::U16(_) => flatgeobuf::ColumnType::UShort,
        FieldValue::U32(_) => flatgeobuf::ColumnType::UInt,
        FieldValue::U64(_) => flatgeobuf::ColumnType::ULong,
        FieldValue::F32(_) => flatgeobuf::ColumnType::Float,
        FieldValue::F64(_) => flatgeobuf::ColumnType::Double,
        FieldValue::Str(_) | FieldValue::BoxedStr(_) => flatgeobuf::ColumnType::String,
        FieldValue::Bytes(_) | FieldValue::BoxedBytes(_) => flatgeobuf::ColumnType::Binary,
    }
}

fn to_column_value<'a>(source: &'a FieldValue<'_>) -> flatgeobuf::geozero::ColumnValue<'a> {
    match source {
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
        FieldValue::BoxedStr(s) => flatgeobuf::geozero::ColumnValue::String(s),
        FieldValue::Bytes(b) => flatgeobuf::geozero::ColumnValue::Binary(b),
        FieldValue::BoxedBytes(b) => flatgeobuf::geozero::ColumnValue::Binary(b),
    }
}
