// 無くせるはず
#[derive(Debug)]
pub enum GeometryRef<'a> {
    Point(&'a geo_types::Point),
    LineString(&'a geo_types::LineString),
    Generic(&'a geo_types::Geometry),
}

pub trait GeometryFormat {
    fn format(&self, geometry: GeometryRef);
}
pub trait Geometry {
    fn serialize(&self, fmt: &impl GeometryFormat);
}
impl Geometry for geo_types::Point {
    fn serialize(&self, fmt: &impl GeometryFormat) {
        fmt.format(GeometryRef::Point(&self));
    }
}

pub trait PropertyValueFormat {
    fn format_i32(self, value: i32);
    fn format_str(self, value: &str);
}
pub trait PropertyValue {
    fn serialize(self, fmt: impl PropertyValueFormat);
}
impl PropertyValue for i32 {
    fn serialize(self, fmt: impl PropertyValueFormat) {
        fmt.format_i32(self);
    }
}
impl PropertyValue for &str {
    fn serialize(self, fmt: impl PropertyValueFormat) {
        fmt.format_str(self);
    }
}
impl PropertyValue for String {
    fn serialize(self, fmt: impl PropertyValueFormat) {
        fmt.format_str(&self);
    }
}
pub trait ProperyFormat {
    fn format(&self, key: &'static str, value: &impl PropertyValue);
}
pub trait Properties {
    fn serialize(&self, fmt: impl ProperyFormat);
}
pub trait Attributes {
    fn serialize_geometry(&mut self, fmt: &impl GeometryFormat) {
        let _ = fmt;
    }
    fn serialize_properties(&mut self, fmt: impl ProperyFormat) {
        let _ = fmt;
    }
}
pub trait Feature: Geometry + Properties {}

#[derive(geoserde::Feature)]
pub struct Child2 {
    #[geometry]
    loc: geo_types::Point,
    count: i32,
}
impl Geometry for Child2 {
    fn serialize(&self, fmt: &impl GeometryFormat) {
        Geometry::serialize(&self.loc, fmt);
    }
}
impl Properties for Child2 {
    fn serialize(&self, fmt: impl ProperyFormat) {
        fmt.format("count", &self.count);
    }
}
impl Feature for Child2 {}

#[derive(geoserde::Feature)]
pub struct MyFeature2 {
    #[geometry]
    child: Child2,
    title: String,
}
impl Geometry for MyFeature2 {
    fn serialize(&self, fmt: &impl GeometryFormat) {
        Geometry::serialize(&self.child, fmt);
    }
}
impl Properties for MyFeature2 {
    fn serialize(&self, fmt: impl ProperyFormat) {
        // fmt.format("child", &self.child); // ここの辻褄が合わない
        fmt.format("title", &self.title);
    }
}

fn main() {}
