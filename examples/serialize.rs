use geo_types::Point;
use geoserde::FeatureSerializer;
use geozero::geojson::GeoJsonWriter;
use serde::Serialize;

#[derive(Serialize)]
struct Station {
    loc: Point,         // geometry
    name: &'static str, // property
    europe: bool,       // property
}

fn main() -> anyhow::Result<()> {
    let features = vec![
        Station {
            loc: Point::new(51.5321, -0.1233),
            name: "King's Cross",
            europe: true,
        },
        Station {
            loc: Point::new(139.7661, 35.6812),
            name: "Tokyo",
            europe: false,
        },
    ];

    let mut buf = vec![];
    let mut json = GeoJsonWriter::new(&mut buf);
    let mut ser = FeatureSerializer::new(&mut json);
    features.serialize(&mut ser)?;

    println!("{}", std::str::from_utf8(&buf)?);
    Ok(())
}
