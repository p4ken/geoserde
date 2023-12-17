#![cfg(feature = "geozero")]

use geo_types::LineString as LS;
use geoserde::{FeatureSerializer, GeometrySerializer, PropertySerializer};
use serde::Serialize;

#[test]
fn geometry_test() -> anyhow::Result<()> {
    let mut buf = Vec::<u8>::new();
    let mut sink = geozero::wkt::WktWriter::new(&mut buf);
    let mut sut = GeometrySerializer::new(&mut sink);
    my_geometry().serialize(&mut sut)?;
    assert_eq!(
        "LINESTRING(139.691667 35.689722,139.7454329 35.6585805)",
        String::from_utf8(buf)?
    );
    Ok(())
}

#[test]
fn property_test() -> anyhow::Result<()> {
    let mut buf = Vec::<u8>::new();
    let mut sink = geozero::geojson::GeoJsonWriter::new(&mut buf);
    let mut sut = PropertySerializer::new(0, "my_property", &mut sink);
    my_property().serialize(&mut sut)?;
    assert_eq!(r#""id": 1, "length": 2.2"#, String::from_utf8(buf)?);
    Ok(())
}

#[test]
fn feature_test() -> anyhow::Result<()> {
    let mut buf = Vec::<u8>::new();
    let mut sink = geozero::geojson::GeoJsonWriter::new(&mut buf);
    let mut sut = FeatureSerializer::new(&mut sink);
    my_feature().serialize(&mut sut)?;
    Ok(())
}

fn my_geometry() -> LS {
    vec![(139.691667, 35.689722), (139.7454329, 35.6585805)].into()
}

fn my_property() -> MyProperty {
    MyProperty { id: 1, length: 2.2 }
}

fn my_feature() -> MyFeature {
    let MyProperty { id, length } = my_property();
    MyFeature {
        geometry: my_geometry(),
        id,
        length,
    }
}

#[derive(Serialize)]
struct MyProperty {
    id: i32,
    length: f64,
}

#[derive(Serialize)]
struct MyFeature {
    id: i32,
    geometry: LS,
    length: f64,
}
