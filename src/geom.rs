#[derive(Debug, Clone, Copy)]
pub enum Container {
    Coord,
    Point,
    _MultiPoint,
    Line,
    LineString { len: usize },
    _MultiLineString,
    Polygon,
    _MultiPolygon,
    _Geometry,
    _GeometryCollection,
}
impl Container {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Coord => "Coord",
            Self::Point => "Point",
            Self::_MultiPoint => "MultiPoint",
            Self::Line => "Line",
            Self::LineString { .. } => "LineString",
            Self::_MultiLineString => "MultiLineString",
            Self::Polygon => "Polygon",
            Self::_MultiPolygon => "MultiPolygon",
            Self::_Geometry => "Geometry",
            Self::_GeometryCollection => "GeometryCollection",
        }
    }
}
