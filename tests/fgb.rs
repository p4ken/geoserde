use std::io::Cursor;

use fgbfile::FgbFile;
use flatgeobuf::{geozero::ToGeo, FallibleStreamingIterator, FeatureProperties, FgbReader};
use geo_types::LineString;
use serde::Serialize;

#[test]
fn serialize_to_fgb() -> anyhow::Result<()> {
    let mut buf = vec![];
    let layer = [
        Feature {
            shape: vec![(11., 21.)].into(),
            rank: 1,
        },
        Feature {
            shape: vec![(12., 22.)].into(),
            rank: 2,
        },
    ];
    let count = FgbFile::new(&mut buf, "my_layer").write_features(&layer)?;
    assert_eq!(count, 2);

    let cursor = Cursor::new(buf);
    let mut fgb_iter = FgbReader::open(cursor)?.select_all()?;
    assert_eq!(Some(2), fgb_iter.features_count());
    assert_eq!(1, fgb_iter.header().columns().unwrap().len());

    let mut fgb_layer = Vec::new();
    while let Some(fgb_feat) = fgb_iter.next()? {
        fgb_layer.push(Feature {
            shape: fgb_feat.to_geo()?.try_into()?,
            rank: fgb_feat.property::<i32>("rank")?,
        });
    }
    assert!(fgb_layer.contains(&layer[0]));
    assert!(fgb_layer.contains(&layer[1]));
    Ok(())
}

#[derive(Serialize, PartialEq)]
struct Feature {
    shape: LineString,
    rank: i32,
}
