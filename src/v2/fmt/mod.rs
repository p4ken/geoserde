// 個別のフォーマット読み込み実装を
// なるべくゼロコピーでgeoserdeのインタフェースに適合させるアダプタ。
// プロパティは serde で、ジオメトリは geo-traits でラップする。
// そうすることで、geoserdeの利用者はこれらのデータフォーマットから、
// DeserializeFeatureを実装したRustオブジェクトへと簡単にデシリアライズできる。

#[cfg(feature = "flatgeobuf")]
pub mod fgb;
#[cfg(feature = "geozero")]
mod geozero;
#[cfg(feature = "geojson")]
pub mod json;
#[cfg(feature = "shapefile")]
pub mod shp;
