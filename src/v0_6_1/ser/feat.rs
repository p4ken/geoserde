pub struct _Feature<G, P> {
    pub geometry: G,
    pub properties: P,
}

pub trait AsFeature {
    type G: geo_traits::GeometryTrait<T = f64>;
    type S: serde::Serialize;
    fn as_geometry(&self) -> Self::G;
    fn as_properties(&self) -> Self::S;
}
