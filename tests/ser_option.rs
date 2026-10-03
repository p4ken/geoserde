use std::borrow::Cow;

use geoserde::ser;
use serde::Serialize;

#[derive(Default)]
struct Collector(Vec<(String, ser::FieldValue<'static>)>);

impl ser::SerializeProperties for &mut Collector {
    type Ok = ();
    type Error = std::convert::Infallible;

    fn serialize_property(
        &mut self,
        key: Cow<'static, str>,
        value: ser::FieldValue<'_>,
    ) -> Result<(), Self::Error> {
        self.0.push((key.into_owned(), value.into_owned()));
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

fn collect(
    source: impl Serialize,
    option: ser::FlattenOption,
) -> Vec<(String, ser::FieldValue<'static>)> {
    let mut sink = Collector::default();
    source
        .serialize(ser::TableSerializer::with_option(&mut sink, option))
        .unwrap();
    sink.0
}

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

    let entries = collect(
        Outer {
            inner: Inner { b: 1 },
        },
        ser::FlattenOption::full().object("/"),
    );
    assert_eq!(entries, vec![("inner/b".into(), ser::FieldValue::I32(1))]);
}

/// `simple_array` changes the separator between joined array elements.
#[test]
fn simple_array_separator() {
    #[derive(Serialize)]
    struct Tags {
        tags: Vec<i32>,
    }

    let entries = collect(
        Tags {
            tags: vec![1, 2, 3],
        },
        ser::FlattenOption::full().simple_array("|"),
    );
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

    let entries = collect(
        Items {
            items: vec![Item { a: 1 }, Item { a: 2 }],
        },
        ser::FlattenOption::full().object_array("<", ">"),
    );
    assert_eq!(
        entries,
        vec![
            ("items<0>.a".into(), ser::FieldValue::I32(1)),
            ("items<1>.a".into(), ser::FieldValue::I32(2)),
        ]
    );
}
