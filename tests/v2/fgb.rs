use flatgeobuf::{FallibleStreamingIterator, FeatureProperties};
use geozero::ToGeo;

#[test]
fn fgb_test() {
    let mut fgb = std::io::Cursor::new("");
    let mut reader = flatgeobuf::FgbReader::open(&mut fgb)
        .unwrap()
        .select_all()
        .unwrap();
    while let Some(feat) = reader.next().unwrap() {
        // ゼロコピー
        let _geom = feat.to_geo().unwrap();
        let _prop = feat.property::<i32>("").unwrap();
    }
}

// fgbfileの問題は、
// - 複雑なジオメトリ実装の限界
// - ジオメトリ検出のための実行時オーバーヘッド
// - 複数ジオメトリがどうなるか想像しにくい
// - プロパティを強制flattenすると子structのジオメトリが検出できない
// - 手動でflattenを付けさせると (悪くない)
// - 2D以外が非サポート
