use serde::Serialize;

mod testing;

/// A scalar value cannot be used as properties (must be struct or map).
#[test]
fn scalar_properties_rejected() {
    let fgb_writer = testing::fgb_writer(flatgeobuf::GeometryType::Point);
    let mut fgb_ser = geoserde_fgb::FeatureSerializer::new(fgb_writer);

    let result = fgb_ser.serialize_feature(testing::p(0), 42_i32);
    assert!(result.is_err());
}

/// A string value cannot be used as properties.
#[test]
fn string_properties_rejected() {
    let fgb_writer = testing::fgb_writer(flatgeobuf::GeometryType::Point);
    let mut fgb_ser = geoserde_fgb::FeatureSerializer::new(fgb_writer);

    let result = fgb_ser.serialize_feature(testing::p(0), "hello");
    assert!(result.is_err());
}

/// A Vec (sequence) cannot be used as properties.
#[test]
fn seq_properties_rejected() {
    let fgb_writer = testing::fgb_writer(flatgeobuf::GeometryType::Point);
    let mut fgb_ser = geoserde_fgb::FeatureSerializer::new(fgb_writer);

    let result = fgb_ser.serialize_feature(testing::p(0), vec![1, 2, 3]);
    assert!(result.is_err());
}

/// A bool value cannot be used as properties.
#[test]
fn bool_properties_rejected() {
    let fgb_writer = testing::fgb_writer(flatgeobuf::GeometryType::Point);
    let mut fgb_ser = geoserde_fgb::FeatureSerializer::new(fgb_writer);

    let result = fgb_ser.serialize_feature(testing::p(0), true);
    assert!(result.is_err());
}

/// Unit type cannot be used as properties.
#[test]
fn unit_properties_rejected() {
    let fgb_writer = testing::fgb_writer(flatgeobuf::GeometryType::Point);
    let mut fgb_ser = geoserde_fgb::FeatureSerializer::new(fgb_writer);

    let result = fgb_ser.serialize_feature(testing::p(0), ());
    assert!(result.is_err());
}

/// A unit struct cannot be used as properties.
#[test]
fn unit_struct_properties_rejected() {
    #[derive(Serialize)]
    struct Empty;

    let fgb_writer = testing::fgb_writer(flatgeobuf::GeometryType::Point);
    let mut fgb_ser = geoserde_fgb::FeatureSerializer::new(fgb_writer);

    let result = fgb_ser.serialize_feature(testing::p(0), &Empty);
    assert!(result.is_err());
}

/// A tuple cannot be used as properties.
#[test]
fn tuple_properties_rejected() {
    let fgb_writer = testing::fgb_writer(flatgeobuf::GeometryType::Point);
    let mut fgb_ser = geoserde_fgb::FeatureSerializer::new(fgb_writer);

    let result = fgb_ser.serialize_feature(testing::p(0), (1, 2));
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
    let mut fgb_ser = geoserde_fgb::FeatureSerializer::new(fgb_writer);

    let result = fgb_ser.serialize_feature(testing::p(0), &Kind::A);
    assert!(result.is_err());
}

/// A map with non-string keys should be rejected.
#[test]
fn non_string_key_map_rejected() {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    map.insert(vec![1, 2], "value");

    let fgb_writer = testing::fgb_writer(flatgeobuf::GeometryType::Point);
    let mut fgb_ser = geoserde_fgb::FeatureSerializer::new(fgb_writer);

    let result = fgb_ser.serialize_feature(testing::p(0), &map);
    assert!(result.is_err());
}

/// A root-level rejection keeps the `TableError` in the source chain.
#[test]
fn root_rejection_source_chain() {
    let fgb_writer = testing::fgb_writer(flatgeobuf::GeometryType::Point);
    let mut fgb_ser = geoserde_fgb::FeatureSerializer::new(fgb_writer);

    let err = fgb_ser
        .serialize_feature(testing::p(0), 42_i32)
        .unwrap_err();
    assert!(matches!(
        err,
        geoserde_fgb::Error::Table(geoserde::ser::TableError::Root)
    ));
    let source = std::error::Error::source(&err).unwrap();
    assert_eq!(source.to_string(), "data source must be a map or struct");
}

/// A non-string map key keeps the key error in the source chain.
#[test]
fn non_string_key_source_chain() {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    map.insert(vec![1, 2], "value");

    let fgb_writer = testing::fgb_writer(flatgeobuf::GeometryType::Point);
    let mut fgb_ser = geoserde_fgb::FeatureSerializer::new(fgb_writer);

    let err = fgb_ser.serialize_feature(testing::p(0), &map).unwrap_err();
    let table = std::error::Error::source(&err).unwrap();
    assert_eq!(table.to_string(), "map key must be a string");
    let key = table.source().unwrap();
    assert_eq!(key.to_string(), "found a nested value");
}

/// A custom error from the user's `Serialize` impl reaches the source chain.
#[test]
fn custom_error_source_chain() {
    struct Failing;

    impl Serialize for Failing {
        fn serialize<S: serde::Serializer>(&self, _: S) -> Result<S::Ok, S::Error> {
            Err(serde::ser::Error::custom("boom"))
        }
    }

    #[derive(Serialize)]
    struct Props {
        a: Failing,
    }

    let fgb_writer = testing::fgb_writer(flatgeobuf::GeometryType::Point);
    let mut fgb_ser = geoserde_fgb::FeatureSerializer::new(fgb_writer);

    let err = fgb_ser
        .serialize_feature(testing::p(0), &Props { a: Failing })
        .unwrap_err();
    assert!(matches!(
        &err,
        geoserde_fgb::Error::Table(geoserde::ser::TableError::Source { key: Some(key), .. })
            if key == "a"
    ));
    let table = std::error::Error::source(&err).unwrap();
    assert_eq!(table.to_string(), "failed to serialize `a`");
    assert_eq!(table.source().unwrap().to_string(), "boom");
}
