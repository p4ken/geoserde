pub struct Coordinate {
    pub x: f64,
    pub y: f64,
    pub z: Option<f64>,
    pub m: Option<f64>,
    /* 任意の座標を追加するのは大変 */
}
impl Coordinate {
    pub fn with_x_y(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
            z: None,
            m: None,
        }
    }
}
// ここまでくると geo_types で良いような・・・
pub enum Geometry<CI: Iterator<Item = Coordinate>, CII: Iterator<Item = CI>> {
    Point(Coordinate),
    LineString(CI),
    Polygon(CII),
}
pub trait GeometrySerializer: Sized {
    fn serialize_point(self, point: &Coordinate);
    fn serialize_line(self, line: [Coordinate; 2]) {
        self.serialize_line_string(line.into_iter())
    }
    fn serialize_line_string(self, line_string: impl Iterator<Item = Coordinate>);
    // 果たして実装できるか？
    fn serialize_polygon(self, rings: impl Iterator<Item = impl Iterator<Item = Coordinate>>) {
        self.serialize_geometry(Geometry::Polygon(rings));
    }
    fn serialize_geometry<CI: Iterator<Item = Coordinate>, CII: Iterator<Item = CI>>(
        self,
        geometry: Geometry<CI, CII>,
    ) {
        self.serialize_geometry_collection(std::iter::once(geometry));
    }
    fn serialize_geometry_collection<CI: Iterator<Item = Coordinate>, CII: Iterator<Item = CI>>(
        self,
        collection: impl Iterator<Item = Geometry<CI, CII>>,
    );
}

pub trait SerializeGeometry {
    fn serialize_geometry<S>(&self, serializer: S)
    where
        S: GeometrySerializer;
}
impl SerializeGeometry for geo_types::Polygon {
    fn serialize_geometry<S>(&self, _serializer: S)
    where
        S: GeometrySerializer,
    {
        // 実に微妙
        todo!()
        // let rings = std::iter::once(
        //     self.exterior()
        //         .into_iter()
        //         .map(|c| Coordinate::with_x_y(c.x, c.y)),
        // )
        // .chain(self.interiors().into_iter().map(|line_string| {
        //     line_string
        //         .into_iter()
        //         .map(|c| Coordinate::with_x_y(c.x, c.y))
        // }));
        // serializer.serialize_polygon(rings)
    }
}

fn main() {}
