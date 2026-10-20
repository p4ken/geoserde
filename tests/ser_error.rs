use std::collections::HashMap;

use geoserde::ser;
use serde::Serialize;

mod testing;

// --- Root ---

/// Anything but a struct or a map is rejected at the root.
#[test]
fn root_rejected() {
    #[derive(Serialize)]
    struct Unit;
    #[derive(Serialize)]
    enum Kind {
        A,
    }

    let err = testing::flatten(42_i32).unwrap_err();
    assert!(matches!(err, ser::TableError::Root));
    assert_eq!(err.to_string(), "data source must be a map or struct");

    assert!(matches!(
        testing::flatten("hello"),
        Err(ser::TableError::Root)
    ));
    assert!(matches!(testing::flatten(true), Err(ser::TableError::Root)));
    assert!(matches!(testing::flatten(()), Err(ser::TableError::Root)));
    assert!(matches!(
        testing::flatten(vec![1, 2]),
        Err(ser::TableError::Root)
    ));
    assert!(matches!(
        testing::flatten((1, 2)),
        Err(ser::TableError::Root)
    ));
    assert!(matches!(testing::flatten(Unit), Err(ser::TableError::Root)));
    assert!(matches!(
        testing::flatten(Kind::A),
        Err(ser::TableError::Root)
    ));
}

// --- Map key ---

/// A non-string map key is rejected.
#[test]
fn non_string_map_key_rejected() {
    let map = HashMap::from([(vec![1, 2], "value")]);

    let err = testing::flatten(&map).unwrap_err();
    let ser::TableError::Key(key_err) = &err else {
        panic!("{err:?}");
    };
    assert_eq!(key_err.parent(), None);
    assert_eq!(err.to_string(), "map key must be a string");
    let source = std::error::Error::source(&err).unwrap();
    assert_eq!(source.to_string(), "found a nested value");
}

/// A `None` map key is rejected.
#[test]
fn none_map_key_rejected() {
    let map = HashMap::from([(None::<&str>, "value")]);

    let err = testing::flatten(&map).unwrap_err();
    let ser::TableError::Key(key_err) = &err else {
        panic!("{err:?}");
    };
    assert_eq!(key_err.parent(), None);
    let source = std::error::Error::source(key_err).unwrap();
    assert_eq!(source.to_string(), "found an empty value");
}

/// A non-string key in a nested map reports the key of the map.
#[test]
fn nested_map_key_error_has_parent() {
    #[derive(Serialize)]
    struct Outer {
        inner: Inner,
    }
    #[derive(Serialize)]
    struct Inner {
        tags: HashMap<Vec<i32>, i32>,
    }

    let err = testing::flatten(&Outer {
        inner: Inner {
            tags: HashMap::from([(vec![1], 1)]),
        },
    })
    .unwrap_err();
    let ser::TableError::Key(key_err) = &err else {
        panic!("{err:?}");
    };
    assert_eq!(key_err.parent(), Some("inner.tags"));
    assert_eq!(err.to_string(), "map key in `inner.tags` must be a string");
}

// --- Value ---

struct Failing;

impl Serialize for Failing {
    fn serialize<S: serde::Serializer>(&self, _: S) -> Result<S::Ok, S::Error> {
        Err(serde::ser::Error::custom("boom"))
    }
}

/// An error from a nested `Serialize` impl reports the flattened key.
#[test]
fn value_error_has_key() {
    #[derive(Serialize)]
    struct Outer {
        ok: i32,
        inner: Inner,
    }
    #[derive(Serialize)]
    struct Inner {
        bad: Failing,
    }

    let err = testing::flatten(&Outer {
        ok: 1,
        inner: Inner { bad: Failing },
    })
    .unwrap_err();
    let ser::TableError::Value(value_err) = &err else {
        panic!("{err:?}");
    };
    assert_eq!(value_err.key(), Some("inner.bad"));
    let source = std::error::Error::source(value_err).unwrap();
    assert_eq!(source.to_string(), "boom");
    assert_eq!(err.to_string(), "failed to serialize `inner.bad`");
}

/// An error from an array element reports the indexed key.
#[test]
fn value_error_in_array_has_index() {
    #[derive(Serialize)]
    struct Props {
        items: Vec<Failing>,
    }

    let err = testing::flatten(&Props {
        items: vec![Failing],
    })
    .unwrap_err();
    let ser::TableError::Value(value_err) = &err else {
        panic!("{err:?}");
    };
    assert_eq!(value_err.key(), Some("items[0]"));
}

/// An error from the root `Serialize` impl has no key.
#[test]
fn value_error_at_root_has_no_key() {
    let err = testing::flatten(&Failing).unwrap_err();
    let ser::TableError::Value(value_err) = &err else {
        panic!("{err:?}");
    };
    assert_eq!(value_err.key(), None);
}
