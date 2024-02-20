use geo_types::Point;
use geoserde::FeatureSerializer;
use geozero::geojson::GeoJsonWriter;
use serde::Serialize;

#[test]
fn serialize_flattened_feature() -> anyhow::Result<()> {
    let mut buf = vec![];
    let mut geojson = GeoJsonWriter::new(&mut buf);
    let mut ser = FeatureSerializer::new(&mut geojson);
    my_features().serialize(&mut ser)?;
    println!("{}", std::str::from_utf8(&buf)?);
    Ok(())
}

fn my_features() -> impl Serialize {
    [MyFeature {
        nest1: NestedGeometry {
            geom: Point::default(),
            count: 0,
        },
    }]
}

#[derive(Serialize)]
struct MyFeature {
    #[serde(flatten)]
    nest1: NestedGeometry,
}

#[derive(Serialize)]
struct NestedGeometry {
    geom: Point,
    count: i32,
}
