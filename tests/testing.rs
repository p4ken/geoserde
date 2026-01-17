pub fn c(i: i32) -> geo_types::Coord {
    let x = f64::from(i);
    [x, x + 0.1].into()
}

pub fn p(i: i32) -> geo_types::Point {
    c(i).into()
}

pub fn ls(i: i32) -> geo_types::LineString {
    line(i).into()
}

pub fn line(i: i32) -> geo_types::Line {
    geo_types::Line::new(c(i), c(i + 1))
}

pub fn rect(i: i32) -> geo_types::Rect {
    geo_types::Rect::new(c(-i), c(i))
}

pub fn triangle(i: i32) -> geo_types::Triangle {
    geo_types::Triangle::new([0.0, 0.0].into(), c(i + 1), c(i + 2))
}

pub fn donut(i: i32) -> geo_types::Polygon {
    let mut donut = rect(i + 2).to_polygon();
    // donut.interiors_push(rect(i + 1).to_polygon().into_inner().0);
    donut
}
