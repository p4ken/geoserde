use flatgeobuf::{FallibleStreamingIterator, FeatureProperties};
use geoserde::v2::de::{DeserializeFeature, ParseFeature};
use geozero::ToGeo;

#[test]
fn test_parse_fgb() {
    let mut fgb = std::io::Cursor::new(include_bytes!("sample/a.fgb"));
    let mut reader = flatgeobuf::FgbReader::open(&mut fgb)
        .unwrap()
        .select_all()
        .unwrap();
    while let Some(feat) = reader.next().unwrap() {
        let point = MyFeature::deserialize_feature(feat);
        dbg!(point);
        // let _geom = feat.to_geo().unwrap();
        // let _prop = feat.property::<i32>("number").unwrap();
    }
}

#[test]
fn test_parse_fgb_only_geom() {
    let mut fgb = std::io::Cursor::new(include_bytes!("sample/a.fgb"));
    let mut reader = flatgeobuf::FgbReader::open(&mut fgb)
        .unwrap()
        .select_all()
        .unwrap();
    while let Some(feat) = reader.next().unwrap() {
        let point = geo_types::Point::deserialize_feature(feat);
        dbg!(point);
    }
}

#[derive(Debug)]
struct MyFeature {
    number: i32,
    point: geo_types::Point,
}
impl DeserializeFeature for MyFeature {
    fn deserialize_feature(fmt: impl ParseFeature) -> Self {
        #[derive(serde::Deserialize)]
        struct Properties {
            number: i32,
        }
        let (geometry, properties) = fmt.parse_feature::<_, Properties>();

        Self {
            point: geometry,
            number: properties.number,
        }
    }
}

// fgbfileの問題は、
// - 複雑なジオメトリ実装の限界
// - ジオメトリ検出のための実行時オーバーヘッド
// - 複数ジオメトリがどうなるか想像しにくい
// - プロパティを強制flattenすると子structのジオメトリが検出できない
// - 手動でflattenを付けさせると (悪くない)
// - 2D以外が非サポート
