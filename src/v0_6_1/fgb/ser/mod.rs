// CLEANUP-v0.6: `geom::GeometrySerializer` is the v0_6_1 serde-driven geometry writer; v0_6_2 writes geometry via geo_traits / process_geometry directly
mod geom;
pub mod prop;

// CLEANUP-v0.6: re-exports of the v0_6_1 serde-driven serializers; v0_6_2 exposes its own FeatureSerializer / LayerSerializer
pub use geom::GeometrySerializer;
// CLEANUP-v0.6: re-exports of the v0_6_1 serde-driven serializers; v0_6_2 exposes its own FeatureSerializer / LayerSerializer
pub use prop::PropertiesSerializer;
