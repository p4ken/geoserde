#[test]
fn geojson_test() {
    const GEOJSON: &str = r#"
{
  "type": "Feature",
  "properties": { "food": "donuts" },
  "geometry": {
    "type": "Point",
    "coordinates": [ -118.2836, 34.0956 ]
  }
}
"#;
    let reader = geojson::FeatureReader::from_reader(GEOJSON.as_bytes());
    // 内部的に geojson::Geometryをはさんでgeo_types だから 1コピー
    // geoserdeのtraitもgeojson::Geometryに実装すれば良いか
    let _ = reader
        .deserialize::<MyStruct>()
        .unwrap()
        .collect::<Vec<_>>();
}

#[derive(serde::Deserialize, geoserde::Feature)]
struct MyStruct {
    #[geometry] // に変わるだけ
    #[serde(deserialize_with = "geojson::de::deserialize_geometry")]
    geometry: geo_types::Point<f64>,
    name: String,
    age: u64,
}
