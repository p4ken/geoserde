#[derive(geoserde::Feature)]
pub struct Child2 {
    // デシリアライズには必須ではない。シリアライズに必須かどうかもデータ形式次第。データ形式によっては2個以上でも良いかも
    #[geometry]
    loc: geo_types::Point,
    count: i32,
}

#[derive(geoserde::Feature)]
pub struct MyFeature2 {
    child: Child2,
    title: String,
}

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
    fn deserialize(_fmt: &impl GeometryFormat) -> Self {
        todo!()
    }
}
impl Geometry for geo_types::Point {
    fn serialize(&self, fmt: &mut impl GeometryFormat) {
        fmt.format_point(&self);
    }
}
impl Geometry for Child2 {
    fn serialize(&self, fmt: &mut impl GeometryFormat) {
        Geometry::serialize(&self.loc, fmt);
    }
}
impl Geometry for String {
    fn serialize(&self, _: &mut impl GeometryFormat) {}
}
impl Geometry for MyFeature2 {
    fn serialize(&self, fmt: &mut impl GeometryFormat) {
        Geometry::serialize(&self.child, fmt);
        Geometry::serialize(&self.title, fmt);
    }
}

pub trait ProperyFormat {
    fn format_i32(&self, key: &'static str, value: i32);
    fn format_str(&self, key: &'static str, value: &str);
    fn parse_str(&self, key: &'static str) -> String;
}
pub trait Properties: Sized {
    fn serialize(&self, key: &'static str, fmt: &impl ProperyFormat);
    fn deserialize(_key: &'static str, _fmt: &impl ProperyFormat) -> Option<Self> {
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
impl Properties for Child2 {
    fn serialize(&self, _key: &'static str, fmt: &impl ProperyFormat) {
        Properties::serialize(&self.loc, "loc", fmt);
        Properties::serialize(&self.count, "count", fmt);
    }
}
impl Properties for MyFeature2 {
    fn serialize(&self, _key: &'static str, fmt: &impl ProperyFormat) {
        Properties::serialize(&self.child, "child", fmt);
        Properties::serialize(&self.title, "title", fmt);
    }
}

pub trait Feature: Geometry + Properties {
    fn deserialize(fmt: &(impl GeometryFormat + ProperyFormat), key: &'static str) -> Self;
}
impl Feature for String {
    fn deserialize(fmt: &(impl GeometryFormat + ProperyFormat), key: &'static str) -> Self {
        // 順序依存の仕様とする
        ProperyFormat::parse_str(fmt, key)
    }
}
impl Feature for Child2 {
    fn deserialize(fmt: &(impl GeometryFormat + ProperyFormat), _: &'static str) -> Self {
        Self {
            loc: <geo_types::Point as Properties>::deserialize("count", fmt)
                .unwrap_or_else(|| <geo_types::Point as Geometry>::deserialize(fmt)),
            count: <i32 as Properties>::deserialize("count", fmt).unwrap(),
        }
    }
}
impl Feature for MyFeature2 {
    // プロパティ内の順序はデータ形式とデータ構造の間で同一とする。（暫定仕様）・・・serdeを使えば良いのでは？
    // serdeのhelperが全て使えるわけではない・・・serdeを使いたいが、しかしジオメトリをスキップする方法がない
    // データ構造の都合で、ジオメトリとプロパティが一度に揃う必要がある。
    fn deserialize(fmt: &(impl GeometryFormat + ProperyFormat), _: &'static str) -> Self {
        // ジオメトリとプロパティのどちらが先かはデータ形式の側が決めたいが・・・
        // Self {
        //     child: <Child2 as Properties>::deserialize("child", fmt)
        //         .unwrap_or_else(|| <Child2 as Geometry>::deserialize(fmt)),
        //     title: <String as Properties>::deserialize("title", fmt).unwrap(),
        // }

        // let visitor =

        Self {
            child: Feature::deserialize(fmt, "child"),
            title: Feature::deserialize(fmt, "title"),
        }
    }
}

fn main() {}
