use std::collections::{BTreeSet, HashMap};
use std::io::Cursor;

use flatgeobuf::FallibleStreamingIterator;
use serde::Serialize;

use geoserde::fgb::LayerSerializer;
use geoserde::ser;

#[derive(Serialize)]
struct Feat {
    name: &'static str,
    #[serde(flatten)]
    extra: HashMap<&'static str, &'static str>,
}

#[test]
fn partial_sort_test() -> anyhow::Result<()> {
    let geom = geo_types::point!(x: 0.0_f64, y: 0.0);

    let feat1 = Feat {
        name: "hello",
        extra: HashMap::from([("z_col", "v1"), ("a_col", "v2")]),
    };
    let feat2 = Feat {
        name: "world",
        extra: HashMap::from([("m_col", "v3"), ("a_col", "v4")]),
    };

    // 1. extra だけを flatten してキーを収集・ソート
    let mut sorted_keys = BTreeSet::new();
    for extra in [&feat1.extra, &feat2.extra] {
        sorted_keys.extend(ser::flatten_keys(extra)?);
    }

    // 2. Feed features into LayerSerializer
    let mut ser = LayerSerializer::new();
    ser.add_feature(geom, &feat1);
    ser.add_feature(geom, &feat2);

    // 3. extra_keys に含まれないキー(ソートしない) → 含まれるキー(ソート済み)
    let mut columns: Vec<String> = ser
        .keys()
        .filter(|k| !sorted_keys.contains(*k))
        .map(|k| k.to_owned())
        .collect();
    columns.extend(sorted_keys);

    assert_eq!(columns, vec!["name", "a_col", "m_col", "z_col"]);

    // 4. Pass column order and write features
    ser.set_columns(columns);
    let mut fgb_writer = flatgeobuf::FgbWriter::create("", flatgeobuf::GeometryType::Point)?;
    ser.write_features(&mut fgb_writer)?;

    // 5. Verify using flatgeobuf directly (no geoserde deserialize)
    let mut buf = Vec::new();
    fgb_writer.write(&mut buf)?;

    let mut reader = flatgeobuf::FgbReader::open(Cursor::new(buf))?.select_all()?;

    // Feature 1 values
    let f1 = reader.next()?.unwrap();
    let p1 = flatgeobuf::geozero::FeatureProperties::properties(f1)?;
    assert_eq!(
        p1,
        HashMap::from([
            ("name".to_owned(), "hello".to_owned()),
            ("a_col".to_owned(), "v2".to_owned()),
            ("z_col".to_owned(), "v1".to_owned()),
        ])
    );

    // Feature 2 values
    let f2 = reader.next()?.unwrap();
    let p2 = flatgeobuf::geozero::FeatureProperties::properties(f2)?;
    assert_eq!(
        p2,
        HashMap::from([
            ("name".to_owned(), "world".to_owned()),
            ("a_col".to_owned(), "v4".to_owned()),
            ("m_col".to_owned(), "v3".to_owned()),
        ])
    );

    assert!(reader.next()?.is_none());
    Ok(())
}
