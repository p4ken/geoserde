#![warn(unreachable_pub)]
#![warn(missing_debug_implementations)]
#![warn(rustdoc::broken_intra_doc_links)]
#![warn(rustdoc::private_intra_doc_links)]
#![doc = include_str!("../README.md")]

pub mod de;
pub mod ser;

pub use de::LayerDeserializer;
pub use ser::LayerSerializer;

pub use flatgeobuf;
