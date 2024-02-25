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
    assert_eq!(
        r#"{"type": "Feature", "geometry": {"type": "Point", "coordinates": [1,2]}, "properties": {"count": 3}}"#,
        std::str::from_utf8(&buf)?
    );
    Ok(())
}

fn my_features() -> impl Serialize {
    [MyFeature {
        nest1: NestedGeometry {
            geom: (1, 2).into(),
            count: 3,
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
    geom: Point<i32>,
    count: i32,
}
