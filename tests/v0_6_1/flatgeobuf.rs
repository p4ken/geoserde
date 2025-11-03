use std::io::Cursor;

use anyhow::Result;
use flatgeobuf::{FallibleStreamingIterator, FgbWriter, FgbWriterOptions, GeometryType};
use geo_traits::to_geo::ToGeoGeometry;
use geoserde::v0_6_1::fgb::FeatureDeserializer;
use geozero::{ColumnValue, FeatureProcessor, GeozeroGeometry, PropertyProcessor};
use serde::Deserialize;

use crate::testing;

#[derive(Debug, Deserialize)]
struct MyFeature {
    number: i32,
    // FIXME: missing field `geom`
    #[serde(with = "geoserde::v0_6_1")]
    geom: geo_types::Point,
}

#[test]
fn de() -> Result<()> {
    let mut fgb_iter = flatgeobuf::FgbReader::open(Cursor::new(fgb_buf()?))?.select_all()?;
    // FgbFeature itself should implement Deserialize because it has header in private field
    let fgb_header = fgb_iter
        .header()
        .columns()
        .unwrap()
        .iter()
        .map(|col| (col.name().to_owned(), col.type_()))
        .collect::<Vec<_>>();
    let mut my_features = vec![];
    while let Some(fgb_feat) = fgb_iter.next()? {
        let my_feat = MyFeature::deserialize(&mut FeatureDeserializer::new(&fgb_header, fgb_feat))?;
        my_features.push(my_feat);
    }
    assert_eq!(my_features[0].number, 1);
    // assert_eq!(my_features[1].number, 2);
    Ok(())
}

// Create in-memory flatgeobuf
fn fgb_buf() -> Result<Vec<u8>> {
    let mut fgb_buf = vec![];
    let fgb_w_opt = FgbWriterOptions {
        write_index: false,
        ..Default::default()
    };
    let mut fgb_w =
        FgbWriter::create_with_options("my_features", GeometryType::LineString, fgb_w_opt)?;

    testing::line(0).to_geometry().process_geom(&mut fgb_w)?;
    fgb_w.property(0, "number", &ColumnValue::Int(1))?;
    // fgb_w.property(1, "text", &ColumnValue::String("one"))?;
    fgb_w.feature_end(0)?;

    // testing::line(1).to_geometry().process_geom(&mut fgb_w)?;
    // fgb_w.property(1, "number", &ColumnValue::Int(2))?;
    // // fgb_w.property(1, "text", &ColumnValue::String("two"))?;
    // fgb_w.feature_end(1)?;

    fgb_w.write(&mut fgb_buf)?;
    Ok(fgb_buf)
}
