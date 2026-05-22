mod geo;

pub trait DeserializeGeometry: Sized {
    fn deserialize_geometry<T: geo_traits::GeometryTrait<T = f64>>(
        source: T,
    ) -> Result<Self, GeometryTypeMismatch>;
}

#[derive(Debug)]
pub struct GeometryTypeMismatch {
    expected: &'static str,
}

impl GeometryTypeMismatch {
    pub fn new(expected: &'static str) -> Self {
        Self { expected }
    }
}

impl std::fmt::Display for GeometryTypeMismatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "expected {} geometry", self.expected)
    }
}

impl std::error::Error for GeometryTypeMismatch {}
