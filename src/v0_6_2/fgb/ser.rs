use std::borrow::Cow;

use flatgeobuf::FgbWriter;
use geo_traits::{
    CoordTrait, GeometryCollectionTrait, GeometryTrait, GeometryType, LineStringTrait, LineTrait,
    MultiLineStringTrait, MultiPointTrait, MultiPolygonTrait, PointTrait, PolygonTrait, RectTrait,
    TriangleTrait,
};

use crate::v0_6_1::ser::{
    FieldValue, FlatProperties, SerializeProperties, TableError, TableSerializer,
};

/// FlatGeobuf layer serializer.
///
/// Collects features in two phases:
/// 1. `add_feature()` flattens each feature's properties and stores them with geometry.
/// 2. After sorting `keys()`, call `write_features()` with an `FgbWriter` to emit all features.
///
/// メモリ最適化:
/// - キー文字列は layer 全体で 1 度だけ heap に持つ（`columns`）
/// - 値は `FieldValue<'static>` (24B) を直接保持（`Box<str>`/`Box<[u8]>` の variant 経由）
/// - 各 feature の entry 列は外側 `Vec<Vec<_>>` ではなく 1 本の flat pool +
///   feature 境界 offset で表現（inner Vec の heap オーバーヘッドを削減）
pub struct LayerSerializer {
    column_idx: std::collections::HashMap<String, u32>,
    columns: Vec<String>,
    /// 各列の型。各列で最初に登場した値の型を採用する。
    column_types: Vec<flatgeobuf::ColumnType>,
    geometries: Vec<geo_types::Geometry<f64>>,
    /// 全 feature の entry を連結した flat pool。
    prop_pool: Vec<(u32, FieldValue<'static>)>,
    /// 各 feature の entry が `prop_pool` 上で終わる位置（cumulative）。
    /// feature `i` の range は `prop_offsets[i]..prop_offsets[i+1]`。
    /// 番兵として最後に `prop_pool.len()` を push する。
    prop_offsets: Vec<u32>,
}

impl LayerSerializer {
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

    /// Add a feature (geometry + properties) to this layer.
    ///
    /// 引数を `impl Into<geo_types::Geometry<f64>>` に閉じることで、
    /// `geo_traits::to_geo::ToGeoGeometry` の blanket impl 経由の trait solver 再帰
    /// (rustc #128887 / georust/geo #1385) を回避している。
    /// 任意の `GeometryTrait` 実装を受けたい場合は呼び出し側で
    /// `geo_types::Geometry::<f64>` に変換してから渡す。
    pub fn add_feature(
        &mut self,
        geometry: impl Into<geo_types::Geometry<f64>>,
        properties: impl serde::Serialize,
    ) {
        let entries = FlatProperties::flatten(properties).unwrap().into_entries();
        for (key, value) in entries {
            let idx = self
                .column_idx
                .get(key.as_ref())
                .copied()
                .unwrap_or_else(|| {
                    let i = self.columns.len() as u32;
                    let owned = key.into_owned();
                    self.columns.push(owned.clone());
                    self.column_types
                        .push(crate::v0_6_1::fgb::ser::prop::to_column_type(&value));
                    self.column_idx.insert(owned, i);
                    i
                });
            self.prop_pool.push((idx, value));
        }
        self.geometries.push(geometry.into());
        self.prop_offsets.push(self.prop_pool.len() as u32);
    }

    /// Returns an iterator over the union of all flattened keys.
    pub fn keys(&self) -> impl Iterator<Item = &str> {
        self.columns.iter().map(|s| s.as_str())
    }

    /// Set the column order to use when writing features.
    ///
    /// `columns` には既存の全列を含めること（順序入れ替えのみ可、列の追加・削除は不可）。
    pub fn set_columns(&mut self, columns: Vec<String>) {
        let new_idx: std::collections::HashMap<String, u32> =
            columns.iter().cloned().zip(0u32..).collect();
        let remap: Vec<u32> = self.columns.iter().map(|k| new_idx[k]).collect();
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
    }

    /// Write all collected features to the FgbWriter using the current key order.
    ///
    /// `FgbWriter::property` は連番 idx でしか列を auto-declare しない仕様のため、
    /// 全 feature を書く前に `add_column` で列を宣言してしまう。
    pub fn write_features(self, writer: &mut FgbWriter<'_>) -> Result<(), Error> {
        for (key, &ty) in self.columns.iter().zip(self.column_types.iter()) {
            writer.add_column(key, ty, |_, _| {});
        }
        for (i, geom) in self.geometries.iter().enumerate() {
            process_geometry(geom, writer)?;
            let start = self.prop_offsets[i] as usize;
            let end = self.prop_offsets[i + 1] as usize;
            for (idx, val) in &self.prop_pool[start..end] {
                flatgeobuf::geozero::PropertyProcessor::property(
                    writer,
                    *idx as usize,
                    self.columns[*idx as usize].as_ref(),
                    &crate::v0_6_1::fgb::ser::prop::to_column_value(val),
                )?;
            }
            flatgeobuf::geozero::FeatureProcessor::feature_end(writer, 0)?;
        }
        Ok(())
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
        // TODO: layer とロジックが重複している。統合or廃止する。
        flatgeobuf::geozero::PropertyProcessor::property(
            &mut self.writer,
            index_to_write,
            key.as_ref(),
            &crate::v0_6_1::fgb::ser::prop::to_column_value(&value),
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
