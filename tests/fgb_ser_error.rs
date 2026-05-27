#![cfg(feature = "fgb")]

use serde::Serialize;

mod testing;

/// A scalar value cannot be used as properties (must be struct or map).
#[test]
fn scalar_properties_rejected() {
    let fgb_writer = testing::fgb_writer(flatgeobuf::GeometryType::Point);
    let mut fgb_ser = geoserde::fgb::FeatureSerializer::new(fgb_writer);

    let result = fgb_ser.serialize_feature(&testing::p(0), &42_i32);
    assert!(result.is_err());
}

/// A string value cannot be used as properties.
#[test]
fn string_properties_rejected() {
    let fgb_writer = testing::fgb_writer(flatgeobuf::GeometryType::Point);
    let mut fgb_ser = geoserde::fgb::FeatureSerializer::new(fgb_writer);

    let result = fgb_ser.serialize_feature(&testing::p(0), "hello");
    assert!(result.is_err());
}

/// A Vec (sequence) cannot be used as properties.
#[test]
fn seq_properties_rejected() {
    let fgb_writer = testing::fgb_writer(flatgeobuf::GeometryType::Point);
    let mut fgb_ser = geoserde::fgb::FeatureSerializer::new(fgb_writer);

    let result = fgb_ser.serialize_feature(&testing::p(0), &vec![1, 2, 3]);
    assert!(result.is_err());
}

/// A bool value cannot be used as properties.
#[test]
fn bool_properties_rejected() {
    let fgb_writer = testing::fgb_writer(flatgeobuf::GeometryType::Point);
    let mut fgb_ser = geoserde::fgb::FeatureSerializer::new(fgb_writer);

    let result = fgb_ser.serialize_feature(&testing::p(0), &true);
    assert!(result.is_err());
}

/// Unit type cannot be used as properties.
#[test]
fn unit_properties_rejected() {
    let fgb_writer = testing::fgb_writer(flatgeobuf::GeometryType::Point);
    let mut fgb_ser = geoserde::fgb::FeatureSerializer::new(fgb_writer);

    let result = fgb_ser.serialize_feature(&testing::p(0), &());
    assert!(result.is_err());
}

/// LayerSerializer also rejects scalar properties in add_feature.
#[test]
fn layer_scalar_properties_rejected() {
    let mut layer = geoserde::fgb::LayerSerializer::new();

    let result = layer.add_feature(&testing::p(0), &42_i32);
    assert!(result.is_err());
}

/// LayerSerializer rejects sequence properties in add_feature.
#[test]
fn layer_seq_properties_rejected() {
    let mut layer = geoserde::fgb::LayerSerializer::new();

    let result = layer.add_feature(&testing::p(0), &vec![1, 2, 3]);
    assert!(result.is_err());
}

/// A unit struct cannot be used as properties.
#[test]
fn unit_struct_properties_rejected() {
    #[derive(Serialize)]
    struct Empty;

    let fgb_writer = testing::fgb_writer(flatgeobuf::GeometryType::Point);
    let mut fgb_ser = geoserde::fgb::FeatureSerializer::new(fgb_writer);

    let result = fgb_ser.serialize_feature(&testing::p(0), &Empty);
    assert!(result.is_err());
}

/// A tuple cannot be used as properties.
#[test]
fn tuple_properties_rejected() {
    let fgb_writer = testing::fgb_writer(flatgeobuf::GeometryType::Point);
    let mut fgb_ser = geoserde::fgb::FeatureSerializer::new(fgb_writer);

    let result = fgb_ser.serialize_feature(&testing::p(0), &(1, 2));
    assert!(result.is_err());
}

/// An enum variant (externally tagged) cannot be used as properties.
#[test]
fn enum_variant_properties_rejected() {
    #[derive(Serialize)]
    enum Kind {
        A,
    }

    let fgb_writer = testing::fgb_writer(flatgeobuf::GeometryType::Point);
    let mut fgb_ser = geoserde::fgb::FeatureSerializer::new(fgb_writer);

    let result = fgb_ser.serialize_feature(&testing::p(0), &Kind::A);
    assert!(result.is_err());
}

/// A map with non-string keys should be rejected.
#[test]
fn non_string_key_map_rejected() {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    map.insert(vec![1, 2], "value");

    let fgb_writer = testing::fgb_writer(flatgeobuf::GeometryType::Point);
    let mut fgb_ser = geoserde::fgb::FeatureSerializer::new(fgb_writer);

    let result = fgb_ser.serialize_feature(&testing::p(0), &map);
    assert!(result.is_err());
}
