use geo_types::LineString;
use geozero::wkt::WktReader;
use serde::Deserialize;

fn main() -> anyhow::Result<()> {
    let wkt = "LINESTRING(139.691667 35.689722,139.7454329 35.6585805)";
    let reader = WktReader(wkt.as_bytes());
    // reader.to_geo();
    let mut de = geoserde::GeometryDeserializer::new(reader);
    let geom = LineString::<f64>::deserialize(&mut de)?;
    dbg!(geom);
    Ok(())
}
