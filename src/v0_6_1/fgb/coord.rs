use serde::de::{
    value::{F64Deserializer, MapAccessDeserializer, StrDeserializer},
    IntoDeserializer, MapAccess,
};

pub struct CoordIter<'de> {
    fbs_geom: flatgeobuf::Geometry<'de>,
    index: usize,
}
impl<'de> CoordIter<'de> {
    pub fn new(fbs_geom: flatgeobuf::Geometry<'de>) -> Self {
        Self { fbs_geom, index: 0 }
    }
}
impl CoordIter<'_> {
    fn xy(&self) -> Option<[f64; 2]> {
        let mut iter = self.fbs_geom.xy()?.iter().skip(self.index * 2);
        Some([iter.next()?, iter.next()?])
    }
    fn z(&self) -> Option<f64> {
        self.fbs_geom.z()?.iter().nth(self.index)
    }
    fn m(&self) -> Option<f64> {
        self.fbs_geom.m()?.iter().nth(self.index)
    }
}
impl Iterator for CoordIter<'_> {
    type Item = CoordMap;

    fn next(&mut self) -> Option<Self::Item> {
        let [x, y] = self.xy()?;
        let z = self.z();
        let m = self.m();
        self.index += 1;
        Some(CoordMap::new(x, y, z, m))
    }
}

// TODO: Rename to crate::PointDeserializer
pub struct CoordMap {
    x: f64,
    y: f64,
    z: Option<f64>,
    m: Option<f64>,
    key: &'static str,
}
impl CoordMap {
    fn new(x: f64, y: f64, z: Option<f64>, m: Option<f64>) -> Self {
        Self {
            x,
            y,
            z,
            m,
            key: "",
        }
    }
}
impl<'de> MapAccess<'de> for CoordMap {
    type Error = serde::de::value::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        self.key = match self.key {
            "" => "x",
            "x" => "y",
            "y" => "z",
            "z" => "m",
            "m" => return Ok(None),
            _ => unreachable!(),
        };
        if self.key == "z" && self.z.is_none() {
            self.key = "m";
        }
        if self.key == "m" && self.m.is_none() {
            return Ok(None);
        }
        seed.deserialize(StrDeserializer::new(self.key)).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        match self.key {
            "x" => seed.deserialize(F64Deserializer::new(self.x)),
            "y" => seed.deserialize(F64Deserializer::new(self.y)),
            "z" => seed.deserialize(F64Deserializer::new(self.z.unwrap())),
            "m" => seed.deserialize(F64Deserializer::new(self.m.unwrap())),
            _ => panic!("no key"),
        }
    }
}
impl<'de> IntoDeserializer<'de> for CoordMap {
    type Deserializer = MapAccessDeserializer<CoordMap>;

    fn into_deserializer(self) -> Self::Deserializer {
        MapAccessDeserializer::new(self)
    }
}
