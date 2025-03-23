use crate::v2::de::{DeserializeGeometry, ParseFeature};
use geojson::Value;
use serde::Deserialize;

impl ParseFeature for geojson::Feature {
    fn parse_feature<G: DeserializeGeometry, P: serde::de::DeserializeOwned>(self) -> (G, P) {
        // 1コピー
        // #[derive(Deserialize)]
        // struct Feature<PP: serde::de::DeserializeOwned> {
        //     geometry: geo_types::Geometry,
        //     #[serde(flatten)]
        //     properties: PP,
        // }

        let g = match &self.geometry.as_ref().unwrap().value {
            // 1コピー
            Value::LineString(x) => x,
            _ => todo!(),
        };
        // 実質ゼロコピー
        let p = self.property("key").unwrap();
        // deserializeできぬ
        todo!()
    }
}
