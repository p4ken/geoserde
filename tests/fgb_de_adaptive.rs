#![cfg(feature = "fgb")]

//! Geometry type conversion tests for deserialization.
//! Covers "flatten single element" cases from the conversion matrix.

use std::io::Cursor;

use flatgeobuf::geozero::{FeatureProcessor, GeozeroGeometry};
use geo_types::Geometry;
use serde::Deserialize;

mod testing;

#[derive(Deserialize)]
struct NoProps {}

/// LineString(1 point pair) → Point: flatten single element
#[test]
fn line_string_to_point() -> anyhow::Result<()> {
    let mut fgb_buf = Vec::new();
    {
        let mut w = testing::fgb_writer(flatgeobuf::GeometryType::LineString);
        Geometry::LineString(testing::ls(0)).process_geom(&mut w)?;
        w.feature_end(0)?;
        w.write(&mut fgb_buf)?;
    }

    let fgb_reader = flatgeobuf::FgbReader::open(Cursor::new(fgb_buf))?;
    let mut fgb_de = geoserde::fgb::FeatureDeserializer::new(fgb_reader)?;

    let (geom, _) = fgb_de.iter::<geo_types::Point, NoProps>().next().unwrap()?;
    assert_eq!(geom, testing::p(0));
    Ok(())
}

/// MultiLineString(1 line) → LineString: flatten single element
#[test]
fn multi_line_string_to_line_string() -> anyhow::Result<()> {
    let single = geo_types::MultiLineString::new(vec![testing::ls(0)]);
    let mut fgb_buf = Vec::new();
    {
        let mut w = testing::fgb_writer(flatgeobuf::GeometryType::MultiLineString);
        Geometry::MultiLineString(single).process_geom(&mut w)?;
        w.feature_end(0)?;
        w.write(&mut fgb_buf)?;
    }

    let fgb_reader = flatgeobuf::FgbReader::open(Cursor::new(fgb_buf))?;
    let mut fgb_de = geoserde::fgb::FeatureDeserializer::new(fgb_reader)?;

    let (geom, _) = fgb_de
        .iter::<geo_types::LineString, NoProps>()
        .next()
        .unwrap()?;
    assert_eq!(geom, testing::ls(0));
    Ok(())
}

/// MultiLineString(1 line) → Point: flatten two levels
#[test]
fn multi_line_string_to_point() -> anyhow::Result<()> {
    let single = geo_types::MultiLineString::new(vec![testing::ls(0)]);
    let mut fgb_buf = Vec::new();
    {
        let mut w = testing::fgb_writer(flatgeobuf::GeometryType::MultiLineString);
        Geometry::MultiLineString(single).process_geom(&mut w)?;
        w.feature_end(0)?;
        w.write(&mut fgb_buf)?;
    }

    let fgb_reader = flatgeobuf::FgbReader::open(Cursor::new(fgb_buf))?;
    let mut fgb_de = geoserde::fgb::FeatureDeserializer::new(fgb_reader)?;

    let (geom, _) = fgb_de.iter::<geo_types::Point, NoProps>().next().unwrap()?;
    assert_eq!(geom, testing::p(0));
    Ok(())
}

/// MultiLineString(1 line) → MultiPoint: flatten single element
#[test]
fn multi_line_string_to_multi_point() -> anyhow::Result<()> {
    let single = geo_types::MultiLineString::new(vec![testing::ls(0)]);
    let mut fgb_buf = Vec::new();
    {
        let mut w = testing::fgb_writer(flatgeobuf::GeometryType::MultiLineString);
        Geometry::MultiLineString(single).process_geom(&mut w)?;
        w.feature_end(0)?;
        w.write(&mut fgb_buf)?;
    }

    let fgb_reader = flatgeobuf::FgbReader::open(Cursor::new(fgb_buf))?;
    let mut fgb_de = geoserde::fgb::FeatureDeserializer::new(fgb_reader)?;

    let (geom, _) = fgb_de
        .iter::<geo_types::MultiPoint, NoProps>()
        .next()
        .unwrap()?;
    assert_eq!(
        geom,
        geo_types::MultiPoint::new(vec![testing::p(0), testing::p(1)])
    );
    Ok(())
}

/// Polygon(1 ring) → Point: flatten single element
#[test]
fn polygon_to_point() -> anyhow::Result<()> {
    let simple = geo_types::Polygon::new(
        geo_types::LineString::from(vec![
            testing::c(0),
            testing::c(1),
            testing::c(2),
            testing::c(0),
        ]),
        vec![],
    );
    let mut fgb_buf = Vec::new();
    {
        let mut w = testing::fgb_writer(flatgeobuf::GeometryType::Polygon);
        Geometry::Polygon(simple).process_geom(&mut w)?;
        w.feature_end(0)?;
        w.write(&mut fgb_buf)?;
    }

    let fgb_reader = flatgeobuf::FgbReader::open(Cursor::new(fgb_buf))?;
    let mut fgb_de = geoserde::fgb::FeatureDeserializer::new(fgb_reader)?;

    let (geom, _) = fgb_de.iter::<geo_types::Point, NoProps>().next().unwrap()?;
    assert_eq!(geom, testing::p(0));
    Ok(())
}

/// Polygon(1 ring) → LineString: flatten single element
#[test]
fn polygon_to_line_string() -> anyhow::Result<()> {
    let ring = geo_types::LineString::from(vec![
        testing::c(0),
        testing::c(1),
        testing::c(2),
        testing::c(0),
    ]);
    let simple = geo_types::Polygon::new(ring.clone(), vec![]);
    let mut fgb_buf = Vec::new();
    {
        let mut w = testing::fgb_writer(flatgeobuf::GeometryType::Polygon);
        Geometry::Polygon(simple).process_geom(&mut w)?;
        w.feature_end(0)?;
        w.write(&mut fgb_buf)?;
    }

    let fgb_reader = flatgeobuf::FgbReader::open(Cursor::new(fgb_buf))?;
    let mut fgb_de = geoserde::fgb::FeatureDeserializer::new(fgb_reader)?;

    let (geom, _) = fgb_de
        .iter::<geo_types::LineString, NoProps>()
        .next()
        .unwrap()?;
    assert_eq!(geom, ring);
    Ok(())
}

/// Polygon → MultiLineString (ok, not flatten)
#[test]
fn polygon_to_multi_line_string() -> anyhow::Result<()> {
    let mut fgb_buf = Vec::new();
    {
        let mut w = testing::fgb_writer(flatgeobuf::GeometryType::Polygon);
        Geometry::Polygon(testing::donut(0)).process_geom(&mut w)?;
        w.feature_end(0)?;
        w.write(&mut fgb_buf)?;
    }

    let fgb_reader = flatgeobuf::FgbReader::open(Cursor::new(fgb_buf))?;
    let mut fgb_de = geoserde::fgb::FeatureDeserializer::new(fgb_reader)?;

    let (geom, _) = fgb_de
        .iter::<geo_types::MultiLineString, NoProps>()
        .next()
        .unwrap()?;
    let donut = testing::donut(0);
    let expected = geo_types::MultiLineString::new(
        std::iter::once(donut.exterior().clone())
            .chain(donut.interiors().iter().cloned())
            .collect(),
    );
    assert_eq!(geom, expected);
    Ok(())
}
