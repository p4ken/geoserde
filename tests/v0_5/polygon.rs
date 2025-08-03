use geo_types::{LineString, Polygon};
use geoserde::GeometrySerializer;
use serde::Serialize;

#[test]
fn polygon_test() -> anyhow::Result<()> {
    let mut buf = Vec::<u8>::new();
    let mut sink = geozero::wkt::WktWriter::new(&mut buf);
    let mut sut = GeometrySerializer::new(&mut sink);
    let polygon = Polygon::new(
        LineString::from(vec![(0., 0.), (1., 1.), (1., 0.), (0., 0.)]),
        vec![],
    );
    polygon.serialize(&mut sut)?;
    assert_eq!("POLYGON((0 0,1 1,1 0,0 0))", String::from_utf8(buf)?);
    Ok(())
}

#[cfg(feature = "geozero")]
#[test]
fn holed_polygon_test() -> anyhow::Result<()> {
    use geoserde::GeometrySerializer;

    let mut buf = Vec::<u8>::new();
    let mut sink = geozero::wkt::WktWriter::new(&mut buf);
    let polygon = Polygon::new(
        LineString::from(vec![(0., 0.), (1., 1.), (1., 0.), (0., 0.)]),
        vec![LineString::from(vec![
            (0.1, 0.1),
            (0.9, 0.9),
            (0.9, 0.1),
            (0.1, 0.1),
        ])],
    );
    let mut sut = GeometrySerializer::new(&mut sink);
    polygon.serialize(&mut sut)?;
    assert_eq!(
        "POLYGON((0 0,1 1,1 0,0 0),(0.1 0.1,0.9 0.9,0.9 0.1,0.1 0.1))",
        String::from_utf8(buf)?
    );
    Ok(())
}
