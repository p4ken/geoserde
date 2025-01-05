use geo_types::Point;
use serde::ser::SerializeStruct;

/// For `serde(with=...)`
pub fn serialize<S: serde::Serializer>(point: &Point, ser: S) -> Result<S::Ok, S::Error> {
    let mut state = ser.serialize_struct("__geoserde_geometry", 2)?;
    state.serialize_field("__geoserde_geometry_field", point)?;
    state.end()
}
