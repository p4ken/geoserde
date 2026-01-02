pub fn p(i: i32) -> geo_types::Point {
    let x = f64::from(i);
    [x, x + 0.1].into()
}

pub fn ls(i: i32) -> geo_types::LineString {
    l(i).into()
}

pub fn l(i: i32) -> geo_types::Line {
    geo_types::Line::new(p(i), p(i + 1))
}

pub fn rect(i: i32) -> geo_types::Rect {
    geo_types::Rect::new(p(i), p(i + 1))
}
