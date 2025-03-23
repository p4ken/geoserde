#[test]
fn shape_test() {
    let mut reader = shapefile::Reader::from_path("todo").unwrap();
    // ジオメトリは1コピーとなる
    for res in reader.iter_shapes_and_records_as::<shapefile::Polyline, MyStruct>() {
        let (geom, prop) = res.unwrap();
        let _geom = geo_types::MultiLineString::from(geom)
            .0
            .into_iter()
            .next()
            .unwrap();
    }
}

#[derive(serde::Deserialize)]
struct MyStruct {
    // ここにジオメトリを入れたいのだ！
}
