// CLEANUP-v0.6: serde-Visitor based geometry deserializers tied to the virtual-enum approach; superseded by v0_6_2 -- whole module is removable before v0.6 release
mod line_string;
mod point;
mod polygon;

pub use line_string::{FromPointSeq, LineStringVisitor};
pub use point::{POINT_FIELDS, PointDeserializer, PointVisitor};
pub use polygon::{FromLineStringSeq, PolygonVisitor};
