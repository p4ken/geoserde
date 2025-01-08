use serde::Deserialize;

pub trait GeometryFormat {
    fn format_point(&mut self, point: &geo_types::Point);
    fn parse_point(&self) -> geo_types::Point;
}
impl<T: geozero::GeomProcessor> GeometryFormat for T {
    fn format_point(&mut self, point: &geo_types::Point) {
        self.point_begin(0).unwrap();
        self.xy(point.x(), point.y(), 0).unwrap();
        self.point_end(0).unwrap();
    }

    fn parse_point(&self) -> geo_types::Point {
        todo!()
    }
}
pub trait Geometry: Sized {
    fn serialize(&self, fmt: &mut impl GeometryFormat);
    fn deserialize(fmt: &impl GeometryFormat) -> Self {
        todo!()
    }
}
impl Geometry for geo_types::Point {
    fn serialize(&self, fmt: &mut impl GeometryFormat) {
        fmt.format_point(&self);
    }
}

pub trait ProperyFormat {
    fn format_i32(&self, key: &'static str, value: i32);
    fn format_str(&self, key: &'static str, value: &str);
}
pub trait Properties: Sized {
    fn serialize(&self, key: &'static str, fmt: &impl ProperyFormat);
    fn deserialize(key: &'static str, fmt: &impl ProperyFormat) -> Option<Self> {
        todo!()
    }
}
impl Properties for i32 {
    fn serialize(&self, key: &'static str, fmt: &impl ProperyFormat) {
        fmt.format_i32(key, *self);
    }
}
impl Properties for String {
    fn serialize(&self, key: &'static str, fmt: &impl ProperyFormat) {
        fmt.format_str(key, self);
    }
}
impl Properties for geo_types::Point {
    fn serialize(&self, _: &'static str, _: &impl ProperyFormat) {}
}
pub trait Feature: Geometry + Properties {
    fn deserialize(fmt: &(impl GeometryFormat + ProperyFormat)) -> Self;
}

#[derive(geoserde::Feature)]
pub struct Child2 {
    #[geometry]
    loc: geo_types::Point,
    count: i32,
}
impl Geometry for Child2 {
    fn serialize(&self, fmt: &mut impl GeometryFormat) {
        Geometry::serialize(&self.loc, fmt);
    }
}
impl Properties for Child2 {
    fn serialize(&self, _key: &'static str, fmt: &impl ProperyFormat) {
        Properties::serialize(&self.loc, "loc", fmt);
        Properties::serialize(&self.count, "count", fmt);
    }
}
impl Feature for Child2 {
    fn deserialize(fmt: &(impl GeometryFormat + ProperyFormat)) -> Self {
        Self {
            loc: <geo_types::Point as Properties>::deserialize("count", fmt)
                .unwrap_or_else(|| <geo_types::Point as Geometry>::deserialize(fmt)),
            count: <i32 as Properties>::deserialize("count", fmt).unwrap(),
        }
    }
}

#[derive(geoserde::Feature)]
pub struct MyFeature2 {
    // デシリアライズには必須ではない。シリアライズに必須かどうかもデータ形式次第。
    #[geometry]
    child: Child2,
    title: String,
}
impl Geometry for MyFeature2 {
    fn serialize(&self, fmt: &mut impl GeometryFormat) {
        Geometry::serialize(&self.child, fmt);
    }
}
impl Properties for MyFeature2 {
    fn serialize(&self, _key: &'static str, fmt: &impl ProperyFormat) {
        Properties::serialize(&self.child, "child", fmt);
        Properties::serialize(&self.title, "title", fmt);
    }
}
impl Feature for MyFeature2 {
    // プロパティ内の順序はデータ形式とデータ構造の間で同一とする。（暫定仕様）・・・serdeを使えば良いのでは？
    // serdeのhelperが全て使えるわけではない・・・serdeを使えば良いのでは？
    // データ構造の都合で、ジオメトリとプロパティが一度に揃う必要がある。
    fn deserialize(fmt: &(impl GeometryFormat + ProperyFormat)) -> Self {
        // child = Some(fmt.parse_property::<Child>("child") || fmt.parse_geometry::<Child>());
        Self {
            // TODO: シリアライズと同じで、ジオメトリとプロパティのどちらが先かはデータ形式の側が決める。
            child: <Child2 as Properties>::deserialize("child", fmt)
                .unwrap_or_else(|| <Child2 as Geometry>::deserialize(fmt)),
            title: <String as Properties>::deserialize("title", fmt).unwrap(),
        }
    }
}

fn main() {}
