#![cfg(feature = "geojson")]

use crate::v0_6::{DeserializeGeometry, DeserializeProperties, ParseFeature};
use geojson::Value;

// ゼロコピー
// serde_json::from_reader

// Featureの時点で1コピー
impl ParseFeature for geojson::Feature {
    fn parse_feature<G: DeserializeGeometry, P: DeserializeProperties>(self) -> (G, P) {
        // +1コピー
        // #[derive(Deserialize)]
        // struct Feature<PP: serde::de::DeserializeOwned> {
        //     geometry: geo_types::Geometry,
        //     #[serde(flatten)]
        //     properties: PP,
        // }

        let g = match &self.geometry.as_ref().unwrap().value {
            // +1コピー
            Value::LineString(x) => x,
            _ => todo!(),
        };

        // 実質ゼロコピー
        let p = self.property("key").unwrap();
        // serde_json::from_value で+1コピー
        todo!()
    }
}
