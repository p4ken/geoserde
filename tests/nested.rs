// TODO: ネストしたクラス。geojsonはany objectだから明示的にflattenが必要。

use geo_types::LineString;
use geoserde::GeoDeserialize;
use serde::Deserialize;

#[derive(GeoDeserialize)]
struct Parent {
    #[flatten]
    wrapped_geometry: Child,
}

#[derive(GeoDeserialize)]
struct Child {
    #[geometry]
    shape: LineString,
}
