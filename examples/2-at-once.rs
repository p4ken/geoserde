#[derive(geoserde::Deserialize)]
pub struct Child2 {
    #[geoserde(geometry)]
    loc: geo_types::Point,
    count: i32,
}

// #[derive(geoserde::Feature)]
pub struct MyFeature2 {
    child: Child2,
    title: String,
}

pub trait Deserialize: Sized {
    fn deserialize<'de>(
        geom_fmt: &mut impl geo_traits::GeometryTrait<T = f64>,
        prop_fmt: &impl serde::de::Deserializer<'de>,
    ) -> Self;
}
impl Deserialize for Child2 {
    fn deserialize<'de>(
        geom_fmt: &mut impl geo_traits::GeometryTrait<T = f64>,
        prop_fmt: &impl serde::de::Deserializer<'de>,
    ) -> Self {
        use geo_traits::to_geo::ToGeoGeometry;
        let geometry = geom_fmt.try_to_geometry().unwrap().try_into().unwrap();
        #[derive(serde::Deserialize)]
        struct Properties {
            count: i32,
        }
        let properties = <Properties as serde::Deserialize>::deserialize(prop_fmt).unwrap();
        Self {
            loc: geometry,
            count: properties.count,
        }
    }
}
impl Deserialize for MyFeature2 {
    fn deserialize<'de>(
        geom_fmt: &mut impl geo_traits::GeometryTrait<T = f64>,
        prop_fmt: &impl serde::de::Deserializer<'de>,
    ) -> Self {
        Self {
            child: Child2::deserialize(&mut geom_fmt, &mut prop_fmt),
            title: String::deserialize(&mut geom_fmt, &mut prop_fmt),
        }
    }
}
impl Deserialize for String {
    fn deserialize<'de>(
        _: &mut impl geo_traits::GeometryTrait<T = f64>,
        prop_fmt: &impl serde::de::Deserializer<'de>,
    ) -> Self {
        <String as serde::Deserialize>::deserialize(prop_fmt).unwrap()
    }
}

pub trait ParseProperty {
    fn i32(&mut self) -> i32;
    fn str(&mut self) -> String;
}
impl<T: ParseProperty> ParseProperty for &mut T {
    fn i32(&mut self) -> i32 {
        (*self).i32()
    }
    fn str(&mut self) -> String {
        (*self).str()
    }
}
pub trait FormatProperty {}

fn main() {}
