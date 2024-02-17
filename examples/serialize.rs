use geo_types::Point;
use geoserde::FeatureSerializer;
use geozero::geojson::GeoJsonWriter;
use serde::Serialize;

// Print two features to the console in GeoJson format
fn main() -> anyhow::Result<()> {
    // If you want to write to a file, use BufWriter<File> instead
    let mut buf = vec![];

    // Any format that has an implementation of geozero::FeatureProcessor can be used, such as
    // geojson, wkt, shp, fgb, etc.
    // For more information, see https://docs.rs/geozero/latest/geozero/
    let mut geojson = GeoJsonWriter::new(&mut buf);

    // Serialize features into GeoJson format
    let mut ser = FeatureSerializer::new(&mut geojson);
    my_features().serialize(&mut ser)?;

    println!("{}", std::str::from_utf8(&buf)?);
    Ok(())
}

// Create feature array
fn my_features() -> impl Serialize {
    [
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
    ]
}

// Geographic feature
#[derive(Serialize)]
struct Station {
    // Geometry
    loc: Point,

    // Properties
    name: &'static str,
    europe: bool,
}
