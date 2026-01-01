#![cfg(feature = "flatgeobuf")]

use std::io::Cursor;

use anyhow::Result;
use flatgeobuf::{FallibleStreamingIterator, FgbReader, FgbWriter, GeometryType};
use geo_traits::to_geo::ToGeoGeometry;
use geoserde::v0_6_1::{fgb::FeatureDeserializer, DeserializeGeometry};
use geozero::{ColumnValue, FeatureProcessor, GeozeroGeometry, PropertyProcessor};
use serde::{de::DeserializeOwned, Deserialize};

use crate::testing;

#[test]
fn point_test() -> Result<()> {
    let mut fgb_buf = vec![];
    {
        let mut fgb_w = FgbWriter::create("my_features", GeometryType::Point)?;

        testing::p(0).to_geometry().process_geom(&mut fgb_w)?;
        fgb_w.property(0, "number", &ColumnValue::Int(1))?;
        fgb_w.feature_end(0)?;

        testing::p(1).to_geometry().process_geom(&mut fgb_w)?;
        fgb_w.property(1, "number", &ColumnValue::Int(2))?;
        fgb_w.feature_end(1)?;

        fgb_w.write(&mut fgb_buf)?;
    }

    #[derive(Debug, Deserialize)]
    struct MyFeature {
        #[serde(with = "geoserde::v0_6_1")]
        #[serde(rename = "geoserde::geometry")]
        geom: geo_types::Point,
    }
    let deserialized = deserialize_features::<MyFeature>(&fgb_buf)?;
    assert_eq!(deserialized[0].geom.x_y(), (0.0, 0.1));
    // assert_eq!(deserialized[1].geom.x_y(), (1.0, 1.1));
    Ok(())
}

#[test]
fn line_string_test() -> Result<()> {
    let mut fgb_buf = vec![];
    {
        let mut fgb_w = FgbWriter::create("my_features", GeometryType::LineString)?;

        testing::ls(0).to_geometry().process_geom(&mut fgb_w)?;
        fgb_w.property(0, "number", &ColumnValue::Int(1))?;
        // fgb_w.property(1, "text", &ColumnValue::String("one"))?;
        fgb_w.feature_end(0)?;

        // testing::line(1).to_geometry().process_geom(&mut fgb_w)?;
        // fgb_w.property(1, "number", &ColumnValue::Int(2))?;
        // // fgb_w.property(1, "text", &ColumnValue::String("two"))?;
        // fgb_w.feature_end(1)?;

        fgb_w.write(&mut fgb_buf)?;
    }

    #[derive(Debug, Deserialize)]
    struct MyFeature {
        number: i32,
        #[serde(with = "geoserde::v0_6_1")]
        #[serde(rename = "geoserde::geometry")]
        geom: geo_types::LineString,
    }

    let deserialized = deserialize_features::<MyFeature>(&fgb_buf)?;
    assert_eq!(deserialized[0].number, 1);
    // assert_eq!(my_features[0].geom.0.len(), 2);
    assert_eq!(deserialized[0].geom.0[0].x_y(), (0.0, 0.1));
    Ok(())
}

fn deserialize_features<T: DeserializeOwned>(fgb_buf: &[u8]) -> Result<Vec<T>> {
    let mut features = vec![];
    let mut fgb_iter = FgbReader::open(Cursor::new(fgb_buf))?.select_all()?;
    let fgb_header = fgb_iter.header().into();
    while let Some(fgb_feat) = fgb_iter.next()? {
        let my_point = T::deserialize(&mut FeatureDeserializer::new(&fgb_header, fgb_feat))?;
        features.push(my_point);
    }
    Ok(features)
}
