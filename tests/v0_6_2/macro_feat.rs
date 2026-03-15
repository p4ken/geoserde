use std::borrow::Cow;

use geo_traits::{GeometryTrait, UnimplementedPoint};
use geoserde::v0_6_2::fgb;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Feat {
    name: Cow<'static, str>,
    child: Child,
}

impl Feat {
    fn serialize(&self, ser: &mut fgb::FeatureSerializer) -> Result<(), fgb::ser::Error> {
        // ser.serialize_feature(&self.child, &self)
        todo!()
    }
}

#[derive(Serialize, Deserialize)]
struct Child {
    value: i32,
    #[serde(skip, default = "geo_types::LineString::empty", rename = "geometry")]
    shape: geo_types::LineString,
}

// impl<'a> GeometryTrait for &'a Child {}

#[test]
#[ignore = "WIP"]
fn ser_test() {
    let feat = Feat {
        name: "Hello".into(),
        child: Child {
            shape: crate::testing::ls(0),
            value: 42,
        },
    };
}
