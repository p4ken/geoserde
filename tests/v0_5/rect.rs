use geo_types::Rect;
use geoserde::GeometrySerializer;
use serde::Serialize;

#[test]
fn serialize_rect() {
    let mut buf = Vec::<u8>::new();
    let mut sink = geozero::wkt::WktWriter::new(&mut buf);
    let mut sut = GeometrySerializer::new(&mut sink);
    let rect = Rect::new([0, 0], [1, 1]);
    rect.serialize(&mut sut).unwrap();
    let wkt = String::from_utf8(buf).unwrap();
    assert_eq!("POLYGON((0 0,0 1,1 1,1 0,0 0))", wkt);
}
