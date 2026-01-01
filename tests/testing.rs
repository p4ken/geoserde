pub fn p(i: i32) -> geo_types::Point {
    let x = f64::from(i);
    [x, x + 0.1].into()
}

pub fn ls(i: i32) -> geo_types::LineString {
    vec![p(i), p(i + 1), p(i + 2)].into()
}
