#![cfg(feature = "geozero")]

use geo_types::Coord;
use geoserde::GeometrySerializer;
use serde::Serialize;

#[test]
fn serialize_coord() {
    let mut buf = Vec::<u8>::new();
    let mut sink = geozero::wkt::WktWriter::new(&mut buf);
    let mut sut = GeometrySerializer::new(&mut sink);
    let rect = Coord::from([0, 1]);
    rect.serialize(&mut sut).unwrap();
    let wkt = String::from_utf8(buf).unwrap();
    assert_eq!("POINT(0 1)", wkt);
}
