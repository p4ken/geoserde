use std::borrow::Cow;

use flatgeobuf::FgbWriter;
use geo_traits::{
    CoordTrait, GeometryCollectionTrait, GeometryTrait, GeometryType, LineStringTrait, LineTrait,
    MultiLineStringTrait, MultiPointTrait, MultiPolygonTrait, PointTrait, PolygonTrait, RectTrait,
    TriangleTrait,
};

use crate::fgb::Error;
use crate::ser::{FieldValue, FlatProperties, SerializeProperties, TableSerializer};

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

    // 【問題】引数を `impl GeometryTrait<T = f64>` にして本体で
    //   `ToGeoGeometry::try_to_geometry(&geometry)` を呼ぶと、
    //   `--release` ビルドのみ以下のコンパイルエラーになる（debug では再現しない）:
    //     error[E0275]: overflow evaluating the requirement
    //       `impl GeometryTrait<T = f64>: GeometryTrait`
    //   原因は rustc #128887 / georust/geo #1385:
    //   `ToGeoGeometry` の blanket impl を解決しようとすると trait solver が
    //   GeometryCollectionTrait の associated type を再帰的に展開して overflow する。
    //
    // 【回避策】引数を `impl Into<geo_types::Geometry<f64>>` に閉じることで
    //   `ToGeoGeometry` の blanket impl を経由しなくて済むようにしている。
    //   任意の `GeometryTrait` 実装を受けたい場合は呼び出し側で
    //   `geo_types::Geometry::<f64>` に変換してから渡す。

    /// Add a feature (geometry + properties) to this layer.
    pub fn add_feature(
        &mut self,
        geometry: impl Into<geo_types::Geometry<f64>>,
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
        self.geometries.push(geometry.into());
        self.prop_offsets.push(self.prop_pool.len() as u32);
        Ok(())
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
    ///
    /// メモリ戦略:
    /// - `geometries` / `prop_pool` を一度 `reverse()` してから `pop()` で末尾から消費。
    ///   `into_iter()` だと IntoIter が backing buffer (capacity × size_of) を関数末尾まで
    ///   保持してしまい、要素 heap を drop しても slot 領域が残って OOM の原因になる。
    /// - 容量が len の倍以上になったら `shrink_to_fit()` で実バッファを縮める（償却 O(N)）。
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

        // pop() で原順序になるよう一度反転（in-place, 追加 alloc なし）。
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

            // 1024 features ごとに backing buffer を縮小して OS にメモリを返す。
            // mmap バックの大きな Vec では shrink_to_fit は realloc コピーを伴わず
            // mremap/munmap による page decommit になるため O(1)。
            // これにより dead capacity は常に最大 1024 slot 分（geometries で 64KB、
            // prop_pool で 32KB×列数）に抑えられ、OOM を回避できる。
            if (i + 1) % 1024 == 0 {
                geometries.shrink_to_fit();
                prop_pool.shrink_to_fit();
            }
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
