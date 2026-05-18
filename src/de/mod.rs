mod geo;

pub trait DeserializeGeometry: Sized {
    fn deserialize_geometry<T: geo_traits::GeometryTrait<T = f64>>(source: T) -> Self;
}
