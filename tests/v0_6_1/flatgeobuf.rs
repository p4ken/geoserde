use std::io::Cursor;

use anyhow::Result;
use flatgeobuf::{FallibleStreamingIterator, FgbFeature, FgbWriter, GeometryType};
use geo_traits::to_geo::ToGeoGeometry;
use geoserde::v0_6_1::fgb::FeatureDeserializer;
use geozero::{ColumnValue, FeatureProcessor, GeozeroGeometry, PropertyProcessor};
use serde::Deserialize;

use crate::testing;

#[derive(Debug, Deserialize)]
struct MyFeature {
    number: i32,
    #[serde(with = "geoserde::v0_6_1")]
    geom: geo_types::Point,
}

#[test]
fn de() -> Result<()> {
    let fgb_r = flatgeobuf::FgbReader::open(Cursor::new(fgb_buf()?))?;
    let mut fgb_iter = fgb_r.select_all()?;
    let mut my_features = vec![];
    while let Some(fgb_feat) = fgb_iter.next()? {
        let my_feat = MyFeature::deserialize(FeatureDeserializer(fgb_feat));
        my_features.push(my_feat);
    }
    Ok(())
}

// Create in-memory flatgeobuf
fn fgb_buf() -> Result<Vec<u8>> {
    let mut fgb_buf = vec![];
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
    Ok(fgb_buf)
}
