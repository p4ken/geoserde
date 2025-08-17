#![cfg(feature = "flatgeobuf")]

use std::io::Cursor;

use flatgeobuf::{geozero::PropertyProcessor, FallibleStreamingIterator, FgbWriter, GeometryType};
use geo_traits::to_geo::ToGeoGeometry;
use geoserde::{
    v0_6::{fgb::FeatureParser, DeserializeFeature},
    FeatureSink, GeoDeserialize,
};
use geozero::{ColumnValue, GeozeroGeometry};

mod testing;

#[test]
fn test_parse_fgb() -> anyhow::Result<()> {
    let mut fgb_buf = vec![];
    // Create in-memory flatgeobuf
    {
        let mut fgb_w = FgbWriter::create("my_features", GeometryType::LineString)?;

        testing::line(0).to_geometry().process_geom(&mut fgb_w)?;
        fgb_w.property(0, "number", &ColumnValue::Int(1))?;
        fgb_w.property(1, "text", &ColumnValue::String("one"))?;
        fgb_w.feature_end(0)?;

        testing::line(1).to_geometry().process_geom(&mut fgb_w)?;
        fgb_w.property(0, "number", &ColumnValue::Int(2))?;
        fgb_w.property(1, "text", &ColumnValue::String("two"))?;
        fgb_w.feature_end(1)?;

        fgb_w.write(&mut fgb_buf)?;
    }
    let fgb_r = flatgeobuf::FgbReader::open(Cursor::new(fgb_buf)).unwrap();
    let mut feat_iter = fgb_r.select_all().unwrap();
    let mut my_features = vec![];
    while let Some(fgb_feat) = feat_iter.next()? {
        let my_feat = MyFeature::deserialize_feature(FeatureParser::new(fgb_feat));
        my_features.push(my_feat);
    }
    my_features.sort_by_key(|elem| elem.number);
    assert_eq!(2, my_features.len());
    assert_eq!(1, my_features[0].number);
    assert_eq!(testing::line(0), my_features[0].shape);

    assert_eq!(2, my_features[1].number);
    assert_eq!(testing::line(1), my_features[1].shape);
    Ok(())
}

#[derive(Debug, GeoDeserialize)]
struct MyFeature {
    number: i32,
    #[geometry]
    shape: geo_types::LineString,
}

// fgbfileの問題は、
// - 複雑なジオメトリ実装の限界
// - ジオメトリ検出のための実行時オーバーヘッド
// - 複数ジオメトリがどうなるか想像しにくい
// - プロパティを強制flattenすると子structのジオメトリが検出できない
// - 手動でflattenを付けさせると (悪くない)
// - 2D以外が非サポート
