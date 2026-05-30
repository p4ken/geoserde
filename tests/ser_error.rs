use geoserde::ser;
use serde::Serialize;

// --- FlatProperties::flatten error cases ---

/// Scalar i32 is rejected at the root level.
#[test]
fn flatten_scalar_rejected() {
    let result = ser::FlatProperties::flatten(&42_i32);
    assert!(result.is_err());
}

/// String is rejected at the root level.
#[test]
fn flatten_string_rejected() {
    let result = ser::FlatProperties::flatten("hello");
    assert!(result.is_err());
}

/// Vec (sequence) is rejected at the root level.
#[test]
fn flatten_seq_rejected() {
    let result = ser::FlatProperties::flatten(&vec![1, 2, 3]);
    assert!(result.is_err());
}

/// Bool is rejected at the root level.
#[test]
fn flatten_bool_rejected() {
    let result = ser::FlatProperties::flatten(&true);
    assert!(result.is_err());
}

/// Unit is rejected at the root level.
#[test]
fn flatten_unit_rejected() {
    let result = ser::FlatProperties::flatten(&());
    assert!(result.is_err());
}

/// Tuple is rejected at the root level.
#[test]
fn flatten_tuple_rejected() {
    let result = ser::FlatProperties::flatten(&(1, 2));
    assert!(result.is_err());
}

/// Unit struct is rejected at the root level.
#[test]
fn flatten_unit_struct_rejected() {
    #[derive(Serialize)]
    struct Empty;

    let result = ser::FlatProperties::flatten(&Empty);
    assert!(result.is_err());
}

/// Enum unit variant is rejected at the root level.
#[test]
fn flatten_enum_variant_rejected() {
    #[derive(Serialize)]
    enum Kind {
        A,
    }

    let result = ser::FlatProperties::flatten(&Kind::A);
    assert!(result.is_err());
}

// --- flatten_keys error cases ---

/// flatten_keys rejects scalar values.
#[test]
fn flatten_keys_scalar_rejected() {
    let result = ser::flatten_keys(&42_i32);
    assert!(result.is_err());
}

/// flatten_keys rejects sequences.
#[test]
fn flatten_keys_seq_rejected() {
    let result = ser::flatten_keys(&vec![1, 2, 3]);
    assert!(result.is_err());
}

// --- Successful edge cases ---

/// An empty struct produces zero entries.
#[test]
fn flatten_empty_struct_ok() {
    #[derive(Serialize)]
    struct EmptyFields {}

    let flat = ser::FlatProperties::flatten(&EmptyFields {}).unwrap();
    assert_eq!(flat.into_entries().len(), 0);
}

/// A map with string keys is accepted.
#[test]
fn flatten_string_key_map_ok() {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    map.insert("key", 42);

    let flat = ser::FlatProperties::flatten(&map).unwrap();
    assert_eq!(flat.into_entries().len(), 1);
}

/// A non-string map key is rejected.
#[test]
fn flatten_non_string_key_map_rejected() {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    map.insert(vec![1, 2], "value");

    let result = ser::FlatProperties::flatten(&map);
    assert!(result.is_err());
}

/// flatten_keys succeeds on an empty struct.
#[test]
fn flatten_keys_empty_struct_ok() {
    #[derive(Serialize)]
    struct EmptyFields {}

    let keys = ser::flatten_keys(&EmptyFields {}).unwrap();
    assert!(keys.is_empty());
}

/// flatten_keys succeeds on a struct with fields.
#[test]
fn flatten_keys_struct_ok() {
    #[derive(Serialize)]
    struct TwoFields {
        alpha: i32,
        beta: String,
    }

    let keys = ser::flatten_keys(&TwoFields {
        alpha: 1,
        beta: "x".into(),
    })
    .unwrap();
    assert_eq!(keys, vec!["alpha", "beta"]);
}

/// Nested struct produces dot-separated keys.
#[test]
fn flatten_nested_struct_keys() {
    #[derive(Serialize)]
    struct Outer {
        a: i32,
        inner: Inner,
    }
    #[derive(Serialize)]
    struct Inner {
        b: i32,
    }

    let flat = ser::FlatProperties::flatten(&Outer {
        a: 1,
        inner: Inner { b: 2 },
    })
    .unwrap();
    let entries = flat.into_entries();
    assert_eq!(entries.len(), 2);
}

/// Option::None fields are omitted from the flattened output.
#[test]
fn flatten_option_none_omitted() {
    #[derive(Serialize)]
    struct WithOption {
        present: i32,
        absent: Option<i32>,
    }

    let flat = ser::FlatProperties::flatten(&WithOption {
        present: 1,
        absent: None,
    })
    .unwrap();
    let entries = flat.into_entries();
    assert_eq!(entries.len(), 1);
}

/// Option::Some fields are included.
#[test]
fn flatten_option_some_included() {
    #[derive(Serialize)]
    struct WithOption {
        present: i32,
        extra: Option<i32>,
    }

    let flat = ser::FlatProperties::flatten(&WithOption {
        present: 1,
        extra: Some(99),
    })
    .unwrap();
    let entries = flat.into_entries();
    assert_eq!(entries.len(), 2);
}
