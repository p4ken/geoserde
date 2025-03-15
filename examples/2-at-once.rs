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

pub trait GeoDeserialize: Sized {
    fn geo_deserialize<'de>(
        geom_fmt: &mut impl geo_traits::GeometryTrait<T = f64>,
        prop_fmt: &mut Option<impl serde::Deserializer<'de>>,
    ) -> Self;
}
impl GeoDeserialize for Child2 {
    fn geo_deserialize<'de>(
        geom_fmt: &mut impl geo_traits::GeometryTrait<T = f64>,
        prop_fmt: &mut Option<impl serde::Deserializer<'de>>,
    ) -> Self {
        use geo_traits::to_geo::ToGeoGeometry;
        let geometry = geom_fmt.try_to_geometry().unwrap().try_into().unwrap();
        #[derive(serde::Deserialize)]
        struct Properties {
            count: i32,
        }
        let properties =
            <Properties as serde::Deserialize>::deserialize(prop_fmt.take().unwrap()).unwrap();
        Self {
            loc: geometry,
            count: properties.count,
        }
    }
}
impl GeoDeserialize for MyFeature2 {
    fn geo_deserialize<'de>(
        geom_fmt: &mut impl geo_traits::GeometryTrait<T = f64>,
        prop_fmt: &mut Option<impl serde::Deserializer<'de>>,
    ) -> Self {
        Self {
            child: Child2::geo_deserialize(geom_fmt, prop_fmt),
            title: String::geo_deserialize(geom_fmt, prop_fmt),
        }
    }
}
impl GeoDeserialize for String {
    fn geo_deserialize<'de>(
        _: &mut impl geo_traits::GeometryTrait<T = f64>,
        prop_fmt: &mut Option<impl serde::Deserializer<'de>>,
    ) -> Self {
        // TODO: 入れ子になったとき、Deserializerでは無理。
        todo!()
        // <String as serde::Deserialize>::deserialize(prop_fmt).unwrap()
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
