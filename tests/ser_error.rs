use geoserde::ser;
use serde::Serialize;

// --- flatten_keys error cases ---

/// Scalar i32 is rejected at the root level.
#[test]
fn flatten_scalar_rejected() {
    let result = ser::flatten_keys(42_i32);
    assert!(result.is_err());
}

/// String is rejected at the root level.
#[test]
fn flatten_string_rejected() {
    let result = ser::flatten_keys("hello");
    assert!(result.is_err());
}

/// Vec (sequence) is rejected at the root level.
#[test]
fn flatten_seq_rejected() {
    let result = ser::flatten_keys(vec![1, 2, 3]);
    assert!(result.is_err());
}

/// Bool is rejected at the root level.
#[test]
fn flatten_bool_rejected() {
    let result = ser::flatten_keys(true);
    assert!(result.is_err());
}

/// Unit is rejected at the root level.
#[test]
fn flatten_unit_rejected() {
    let result = ser::flatten_keys(());
    assert!(result.is_err());
}

/// Tuple is rejected at the root level.
#[test]
fn flatten_tuple_rejected() {
    let result = ser::flatten_keys((1, 2));
    assert!(result.is_err());
}

/// Unit struct is rejected at the root level.
#[test]
fn flatten_unit_struct_rejected() {
    #[derive(Serialize)]
    struct Empty;

    let result = ser::flatten_keys(&Empty);
    assert!(result.is_err());
}

/// Enum unit variant is rejected at the root level.
#[test]
fn flatten_enum_variant_rejected() {
    #[derive(Serialize)]
    enum Kind {
        A,
    }

    let result = ser::flatten_keys(&Kind::A);
    assert!(result.is_err());
}

// --- Successful edge cases ---

/// An empty struct produces no keys.
#[test]
fn flatten_empty_struct_ok() {
    #[derive(Serialize)]
    struct EmptyFields {}

    let keys = ser::flatten_keys(&EmptyFields {}).unwrap();
    assert!(keys.is_empty());
}

/// A map with string keys is accepted.
#[test]
fn flatten_string_key_map_ok() {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    map.insert("key", 42);

    let keys = ser::flatten_keys(&map).unwrap();
    assert_eq!(keys, vec!["key"]);
}

/// A non-string map key is rejected.
#[test]
fn flatten_non_string_key_map_rejected() {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    map.insert(vec![1, 2], "value");

    let result = ser::flatten_keys(&map);
    assert!(matches!(
        result,
        Err(ser::TableError::Key {
            parent: None,
            source: ser::StringifyError::Nested,
        })
    ));
}

/// A `None` map key is rejected.
#[test]
fn flatten_none_key_map_rejected() {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    map.insert(None::<&str>, "value");

    let result = ser::flatten_keys(&map);
    assert!(matches!(
        result,
        Err(ser::TableError::Key {
            parent: None,
            source: ser::StringifyError::Empty,
        })
    ));
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

    let keys = ser::flatten_keys(&Outer {
        a: 1,
        inner: Inner { b: 2 },
    })
    .unwrap();
    assert_eq!(keys, vec!["a", "inner.b"]);
}

/// Option::None fields are omitted from the flattened output.
#[test]
fn flatten_option_none_omitted() {
    #[derive(Serialize)]
    struct WithOption {
        present: i32,
        absent: Option<i32>,
    }

    let keys = ser::flatten_keys(&WithOption {
        present: 1,
        absent: None,
    })
    .unwrap();
    assert_eq!(keys, vec!["present"]);
}

/// Option::Some fields are included.
#[test]
fn flatten_option_some_included() {
    #[derive(Serialize)]
    struct WithOption {
        present: i32,
        extra: Option<i32>,
    }

    let keys = ser::flatten_keys(&WithOption {
        present: 1,
        extra: Some(99),
    })
    .unwrap();
    assert_eq!(keys, vec!["present", "extra"]);
}

// --- Error context ---

struct Failing;

impl Serialize for Failing {
    fn serialize<S: serde::Serializer>(&self, _: S) -> Result<S::Ok, S::Error> {
        Err(serde::ser::Error::custom("boom"))
    }
}

/// A non-string key in a nested map reports the key of the map.
#[test]
fn nested_map_key_error_has_parent() {
    use std::collections::HashMap;

    #[derive(Serialize)]
    struct Outer {
        inner: Inner,
    }
    #[derive(Serialize)]
    struct Inner {
        tags: HashMap<Vec<i32>, i32>,
    }

    let err = ser::flatten_keys(&Outer {
        inner: Inner {
            tags: HashMap::from([(vec![1], 1)]),
        },
    })
    .unwrap_err();
    let ser::TableError::Key { parent, .. } = &err else {
        panic!("{err:?}");
    };
    assert_eq!(parent.as_deref(), Some("inner.tags"));
    assert_eq!(err.to_string(), "map key in `inner.tags` must be a string");
}

/// An error from a nested `Serialize` impl reports the flattened key.
#[test]
fn source_error_has_key() {
    #[derive(Serialize)]
    struct Outer {
        ok: i32,
        inner: Inner,
    }
    #[derive(Serialize)]
    struct Inner {
        bad: Failing,
    }

    let err = ser::flatten_keys(&Outer {
        ok: 1,
        inner: Inner { bad: Failing },
    })
    .unwrap_err();
    let ser::TableError::Source { key, source } = &err else {
        panic!("{err:?}");
    };
    assert_eq!(key.as_deref(), Some("inner.bad"));
    assert_eq!(source.to_string(), "boom");
}

/// An error from an array element reports the indexed key.
#[test]
fn source_error_in_array_has_index() {
    #[derive(Serialize)]
    struct Props {
        items: Vec<Failing>,
    }

    let err = ser::flatten_keys(&Props {
        items: vec![Failing],
    })
    .unwrap_err();
    let ser::TableError::Source { key, .. } = &err else {
        panic!("{err:?}");
    };
    assert_eq!(key.as_deref(), Some("items[0]"));
}

/// An error from the root `Serialize` impl has no key.
#[test]
fn source_error_at_root_has_no_key() {
    let err = ser::flatten_keys(&Failing).unwrap_err();
    assert!(matches!(err, ser::TableError::Source { key: None, .. }));
}
