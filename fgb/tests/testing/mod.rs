#![allow(dead_code)]

pub fn c(i: i32) -> geo_types::Coord {
    let x = f64::from(i);
    [x, x + 0.1].into()
}

pub fn p(i: i32) -> geo_types::Point {
    c(i).into()
}

pub fn ls(i: i32) -> geo_types::LineString {
    line(i).into()
}

pub fn line(i: i32) -> geo_types::Line {
    geo_types::Line::new(c(i), c(i + 1))
}

pub fn rect(i: i32) -> geo_types::Rect {
    geo_types::Rect::new(c(-i), c(i))
}

pub fn triangle(i: i32) -> geo_types::Triangle {
    geo_types::Triangle::new([0.0, 0.0].into(), c(i + 1), c(i + 2))
}

pub fn multi_p(i: i32) -> geo_types::MultiPoint {
    geo_types::MultiPoint::new(vec![p(i), p(i + 1)])
}

pub fn multi_ls(i: i32) -> geo_types::MultiLineString {
    geo_types::MultiLineString::new(vec![ls(i), ls(i + 1)])
}

pub fn donut(i: i32) -> geo_types::Polygon {
    let mut donut = rect(i + 2).to_polygon();
    donut.interiors_push(rect(i + 1).to_polygon().into_inner().0);
    donut
}

pub fn fgb_writer(geom_type: flatgeobuf::GeometryType) -> flatgeobuf::FgbWriter<'static> {
    let opt = flatgeobuf::FgbWriterOptions {
        write_index: false,
        ..Default::default()
    };
    flatgeobuf::FgbWriter::create_with_options("", geom_type, opt).unwrap()
}

pub fn layer_ser(
    geom_type: geoserde_fgb::flatgeobuf::GeometryType,
) -> geoserde_fgb::LayerSerializer {
    geoserde_fgb::LayerSerializer::options()
        .geometry_type(geom_type)
        .index(false)
        .create()
        .unwrap()
}

/// `geo_types::Point` cannot be empty, so the source implements `PointTrait` on its own.
#[derive(Clone, Copy)]
pub struct MaybePoint(pub Option<geo_types::Coord>);

impl geo_traits::PointTrait for MaybePoint {
    type CoordType<'a> = geo_types::Coord;

    fn coord(&self) -> Option<Self::CoordType<'_>> {
        self.0
    }
}

impl geo_traits::GeometryTrait for MaybePoint {
    type T = f64;
    type PointType<'a> = MaybePoint;
    type LineStringType<'a> = geo_traits::UnimplementedLineString<f64>;
    type PolygonType<'a> = geo_traits::UnimplementedPolygon<f64>;
    type MultiPointType<'a> = geo_traits::UnimplementedMultiPoint<f64>;
    type MultiLineStringType<'a> = geo_traits::UnimplementedMultiLineString<f64>;
    type MultiPolygonType<'a> = geo_traits::UnimplementedMultiPolygon<f64>;
    type GeometryCollectionType<'a> = geo_traits::UnimplementedGeometryCollection<f64>;
    type RectType<'a> = geo_traits::UnimplementedRect<f64>;
    type TriangleType<'a> = geo_traits::UnimplementedTriangle<f64>;
    type LineType<'a> = geo_traits::UnimplementedLine<f64>;

    fn dim(&self) -> geo_traits::Dimensions {
        geo_traits::Dimensions::Xy
    }

    fn as_type(
        &self,
    ) -> geo_traits::GeometryType<
        '_,
        Self::PointType<'_>,
        Self::LineStringType<'_>,
        Self::PolygonType<'_>,
        Self::MultiPointType<'_>,
        Self::MultiLineStringType<'_>,
        Self::MultiPolygonType<'_>,
        Self::GeometryCollectionType<'_>,
        Self::RectType<'_>,
        Self::TriangleType<'_>,
        Self::LineType<'_>,
    > {
        geo_traits::GeometryType::Point(self)
    }
}

/// `geo_types::MultiPoint` cannot have an empty point either.
pub struct MaybeMultiPoint(pub Vec<MaybePoint>);

impl geo_traits::MultiPointTrait for MaybeMultiPoint {
    type InnerPointType<'a> = MaybePoint;

    fn num_points(&self) -> usize {
        self.0.len()
    }

    unsafe fn point_unchecked(&self, i: usize) -> Self::InnerPointType<'_> {
        self.0[i]
    }
}

impl geo_traits::GeometryTrait for MaybeMultiPoint {
    type T = f64;
    type PointType<'a> = geo_traits::UnimplementedPoint<f64>;
    type LineStringType<'a> = geo_traits::UnimplementedLineString<f64>;
    type PolygonType<'a> = geo_traits::UnimplementedPolygon<f64>;
    type MultiPointType<'a> = MaybeMultiPoint;
    type MultiLineStringType<'a> = geo_traits::UnimplementedMultiLineString<f64>;
    type MultiPolygonType<'a> = geo_traits::UnimplementedMultiPolygon<f64>;
    type GeometryCollectionType<'a> = geo_traits::UnimplementedGeometryCollection<f64>;
    type RectType<'a> = geo_traits::UnimplementedRect<f64>;
    type TriangleType<'a> = geo_traits::UnimplementedTriangle<f64>;
    type LineType<'a> = geo_traits::UnimplementedLine<f64>;

    fn dim(&self) -> geo_traits::Dimensions {
        geo_traits::Dimensions::Xy
    }

    fn as_type(
        &self,
    ) -> geo_traits::GeometryType<
        '_,
        Self::PointType<'_>,
        Self::LineStringType<'_>,
        Self::PolygonType<'_>,
        Self::MultiPointType<'_>,
        Self::MultiLineStringType<'_>,
        Self::MultiPolygonType<'_>,
        Self::GeometryCollectionType<'_>,
        Self::RectType<'_>,
        Self::TriangleType<'_>,
        Self::LineType<'_>,
    > {
        geo_traits::GeometryType::MultiPoint(self)
    }
}

/// `geo_types` has no Z or M, so the source implements `CoordTrait` on its own.
#[derive(Clone, Copy)]
pub struct DimCoord {
    pub dim: geo_traits::Dimensions,
    pub values: [f64; 4],
}

impl geo_traits::CoordTrait for DimCoord {
    type T = f64;

    fn dim(&self) -> geo_traits::Dimensions {
        self.dim
    }

    fn x(&self) -> f64 {
        self.values[0]
    }

    fn y(&self) -> f64 {
        self.values[1]
    }

    fn nth_or_panic(&self, n: usize) -> f64 {
        assert!(n < self.dim.size());
        self.values[n]
    }
}

pub struct DimLineString(pub Vec<DimCoord>);

impl geo_traits::LineStringTrait for DimLineString {
    type CoordType<'a> = DimCoord;

    fn num_coords(&self) -> usize {
        self.0.len()
    }

    unsafe fn coord_unchecked(&self, i: usize) -> Self::CoordType<'_> {
        self.0[i]
    }
}

impl geo_traits::GeometryTrait for DimLineString {
    type T = f64;
    type PointType<'a> = geo_traits::UnimplementedPoint<f64>;
    type LineStringType<'a> = DimLineString;
    type PolygonType<'a> = geo_traits::UnimplementedPolygon<f64>;
    type MultiPointType<'a> = geo_traits::UnimplementedMultiPoint<f64>;
    type MultiLineStringType<'a> = geo_traits::UnimplementedMultiLineString<f64>;
    type MultiPolygonType<'a> = geo_traits::UnimplementedMultiPolygon<f64>;
    type GeometryCollectionType<'a> = geo_traits::UnimplementedGeometryCollection<f64>;
    type RectType<'a> = geo_traits::UnimplementedRect<f64>;
    type TriangleType<'a> = geo_traits::UnimplementedTriangle<f64>;
    type LineType<'a> = geo_traits::UnimplementedLine<f64>;

    fn dim(&self) -> geo_traits::Dimensions {
        self.0[0].dim
    }

    fn as_type(
        &self,
    ) -> geo_traits::GeometryType<
        '_,
        Self::PointType<'_>,
        Self::LineStringType<'_>,
        Self::PolygonType<'_>,
        Self::MultiPointType<'_>,
        Self::MultiLineStringType<'_>,
        Self::MultiPolygonType<'_>,
        Self::GeometryCollectionType<'_>,
        Self::RectType<'_>,
        Self::TriangleType<'_>,
        Self::LineType<'_>,
    > {
        geo_traits::GeometryType::LineString(self)
    }
}

pub fn dim_ls(dim: geo_traits::Dimensions) -> DimLineString {
    DimLineString(
        (0..2)
            .map(|i| {
                let v = f64::from(i);
                DimCoord {
                    dim,
                    values: [v, v + 0.1, v + 0.2, v + 0.3],
                }
            })
            .collect(),
    )
}
