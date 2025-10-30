pub fn line(i: i32) -> geo_types::LineString {
    let org = i as f64;
    vec![[org, org + 0.1], [org + 0.2, org + 0.3]].into()
}
