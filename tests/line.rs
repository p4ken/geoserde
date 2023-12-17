#![cfg(feature = "geozero")]

use geo_types::Line;
use geoserde::GeometrySerializer;
use serde::Serialize;

#[test]
fn line_test() -> anyhow::Result<()> {
    let mut buf = Vec::<u8>::new();
    let mut sink = geozero::wkt::WktWriter::new(&mut buf);
    let mut sut = GeometrySerializer::new(&mut sink);
    let line = Line::from([(139.691667, 35.689722), (139.7454329, 35.6585805)]);
    line.serialize(&mut sut)?;
    assert_eq!(
        "LINESTRING(139.691667 35.689722,139.7454329 35.6585805)",
        String::from_utf8(buf)?
    );
    Ok(())
}
