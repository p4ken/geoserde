#![cfg(feature = "geozero")]

use geo_types::Point;
use geoserde::{FeatureSerializer, GeometrySerializer};
use geozero::{geojson::GeoJsonWriter, wkt::WktWriter};
use serde::Serialize;

#[test]
fn geometry_test() -> anyhow::Result<()> {
    let mut buf = Vec::<u8>::new();
    let mut sink = WktWriter::new(&mut buf);
    let mut sut = GeometrySerializer::new(&mut sink);
    point_0().serialize(&mut sut)?;
    assert_eq!("POINT(139.5860139 35.4813408)", String::from_utf8(buf)?);
    Ok(())
}

#[test]
fn feature_test() -> anyhow::Result<()> {
    let mut buf = Vec::<u8>::new();
    let mut sink = GeoJsonWriter::new(&mut buf);
    let mut sut = FeatureSerializer::new(&mut sink);
    feature_0().serialize(&mut sut)?;
    assert_eq!(1, sut.len());
    assert_eq!(
        r#"{"type": "Feature", "properties": {"id": "ID0"}, "geometry": {"type": "Point", "coordinates": [139.5860139,35.4813408]}}"#,
        String::from_utf8(buf)?
    );
    Ok(())
}

#[test]
pub fn layer_test() -> anyhow::Result<()> {
    let mut buf = Vec::<u8>::new();
    let mut sink = GeoJsonWriter::new(&mut buf);
    let mut sut = FeatureSerializer::new(&mut sink);
    let layer = vec![feature_0(), feature_1()];
    layer.serialize(&mut sut)?;
    assert_eq!(2, sut.len());
    assert_eq!(
        r#"{"type": "Feature", "properties": {"id": "ID0"}, "geometry": {"type": "Point", "coordinates": [139.5860139,35.4813408]}},
{"type": "Feature", "properties": {"id": "ID1"}, "geometry": {"type": "Point", "coordinates": [139.7454329,35.6585805]}}"#,
        String::from_utf8(buf)?
    );
    Ok(())
}

fn point_0() -> Point {
    Point::new(139.5860139, 35.4813408)
}

fn feature_0() -> MyFeature {
    MyFeature {
        id: "ID0",
        pos: point_0(),
    }
}

fn feature_1() -> MyFeature {
    MyFeature {
        id: "ID1",
        pos: Point::new(139.7454329, 35.6585805),
    }
}

#[derive(Serialize)]
struct MyFeature {
    id: &'static str,
    pos: geo_types::Point,
}
