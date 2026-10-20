use geoserde::ser;
use serde::Serialize;

mod testing;

/// `object` changes the separator between nested attribute keys.
#[test]
fn object_separator() {
    #[derive(Serialize)]
    struct Outer {
        inner: Inner,
    }
    #[derive(Serialize)]
    struct Inner {
        b: i32,
    }

    let entries = testing::flatten_with_options(
        Outer {
            inner: Inner { b: 1 },
        },
        &ser::FlattenOptions::full().object("/"),
    )
    .unwrap();
    assert_eq!(entries, vec![("inner/b".into(), ser::FieldValue::I32(1))]);
}

/// `simple_array` changes the separator between joined array elements.
#[test]
fn simple_array_separator() {
    #[derive(Serialize)]
    struct Tags {
        tags: Vec<i32>,
    }

    let entries = testing::flatten_with_options(
        Tags {
            tags: vec![1, 2, 3],
        },
        &ser::FlattenOptions::full().simple_array("|"),
    )
    .unwrap();
    assert_eq!(
        entries,
        vec![("tags".into(), ser::FieldValue::Str("1|2|3".into()))]
    );
}

/// `object_array` changes the brackets around array indices.
#[test]
fn object_array_brackets() {
    #[derive(Serialize)]
    struct Items {
        items: Vec<Item>,
    }
    #[derive(Serialize)]
    struct Item {
        a: i32,
    }

    let entries = testing::flatten_with_options(
        Items {
            items: vec![Item { a: 1 }, Item { a: 2 }],
        },
        &ser::FlattenOptions::full().object_array("<", ">"),
    )
    .unwrap();
    assert_eq!(
        entries,
        vec![
            ("items<0>.a".into(), ser::FieldValue::I32(1)),
            ("items<1>.a".into(), ser::FieldValue::I32(2)),
        ]
    );
}

/// A separator can be a `String` built at runtime.
#[test]
fn owned_separator() {
    #[derive(Serialize)]
    struct Outer {
        inner: Inner,
    }
    #[derive(Serialize)]
    struct Inner {
        b: i32,
    }

    let sep = String::from("::");
    let entries = testing::flatten_with_options(
        Outer {
            inner: Inner { b: 1 },
        },
        &ser::FlattenOptions::full().object(sep),
    )
    .unwrap();
    assert_eq!(entries, vec![("inner::b".into(), ser::FieldValue::I32(1))]);
}
