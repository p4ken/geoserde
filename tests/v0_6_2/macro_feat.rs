use std::borrow::Cow;

use geoserde::v0_6_1::ser::prop::SerializeProperties;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Feat {
    name: Cow<'static, str>,
    #[serde(skip, default = "geo_types::LineString::empty", rename = "geometry")]
    shape: geo_types::LineString,
    value: i32,
}

impl Feat {
    fn serialize(
        &self,
        ser: &mut geoserde::v0_6_2::fgb::FeatureSerializer,
    ) -> Result<(), geoserde::v0_6_2::fgb::ser::Error> {
        #[derive(Serialize)]
        struct Prop<'a> {
            name: &'a Cow<'static, str>,
            value: &'a i32,
        }
        let prop = Prop {
            name: &self.name,
            value: &self.value,
        };
        ser.serialize_feature(&self.shape, prop)
    }
}

#[test]
fn ser_test() {
    let feat = Feat {
        name: "Hello".into(),
        shape: crate::testing::ls(0),
        value: 42,
    };
}
