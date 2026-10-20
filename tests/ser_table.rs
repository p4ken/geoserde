//! Flattening rules listed in the `ser` module docs.

use std::collections::{BTreeMap, HashMap};

use geoserde::ser;
use serde::Serialize;

mod testing;

/// A struct without fields has no properties.
#[test]
fn empty_struct() {
    #[derive(Serialize)]
    struct Props {}

    assert_eq!(testing::flatten(&Props {}).unwrap(), vec![]);
}

/// A scalar field is written under its name, typed after the Rust type.
#[test]
fn scalar() {
    #[derive(Serialize)]
    struct Props {
        name: &'static str,
        count: i32,
        ratio: f64,
        open: bool,
    }

    let entries = testing::flatten(&Props {
        name: "Tokyo",
        count: 1,
        ratio: 0.5,
        open: true,
    })
    .unwrap();
    assert_eq!(
        entries,
        vec![
            ("name".into(), ser::FieldValue::Str("Tokyo".into())),
            ("count".into(), ser::FieldValue::I32(1)),
            ("ratio".into(), ser::FieldValue::F64(0.5)),
            ("open".into(), ser::FieldValue::Bool(true)),
        ]
    );
}

/// A nested struct has its fields under the field name.
#[test]
fn nested_struct() {
    #[derive(Serialize)]
    struct Props {
        name: &'static str,
        address: Address,
    }
    #[derive(Serialize)]
    struct Address {
        city: &'static str,
    }

    let entries = testing::flatten(&Props {
        name: "Tokyo",
        address: Address { city: "Chiyoda" },
    })
    .unwrap();
    assert_eq!(
        entries,
        vec![
            ("name".into(), ser::FieldValue::Str("Tokyo".into())),
            (
                "address.city".into(),
                ser::FieldValue::Str("Chiyoda".into())
            ),
        ]
    );
}

/// `#[serde(flatten)]` merges the fields into the parent without a prefix.
#[test]
fn serde_flatten() {
    #[derive(Serialize)]
    struct Props {
        name: &'static str,
        #[serde(flatten)]
        address: Address,
    }
    #[derive(Serialize)]
    struct Address {
        city: &'static str,
    }

    let entries = testing::flatten(&Props {
        name: "Tokyo",
        address: Address { city: "Chiyoda" },
    })
    .unwrap();
    assert_eq!(
        entries,
        vec![
            ("name".into(), ser::FieldValue::Str("Tokyo".into())),
            ("city".into(), ser::FieldValue::Str("Chiyoda".into())),
        ]
    );
}

/// A map as the properties writes each entry under its key.
#[test]
fn root_map() {
    let map = HashMap::from([("count", 1)]);

    let entries = testing::flatten(&map).unwrap();
    assert_eq!(entries, vec![("count".into(), ser::FieldValue::I32(1))]);
}

/// A nested map has its entries under the field name.
#[test]
fn nested_map() {
    #[derive(Serialize)]
    struct Props {
        count: BTreeMap<&'static str, i32>,
    }

    let entries = testing::flatten(&Props {
        count: BTreeMap::from([("a", 1), ("b", 2)]),
    })
    .unwrap();
    assert_eq!(
        entries,
        vec![
            ("count.a".into(), ser::FieldValue::I32(1)),
            ("count.b".into(), ser::FieldValue::I32(2)),
        ]
    );
}

/// A scalar map key is written as a string.
#[test]
fn scalar_map_key() {
    let map = HashMap::from([(7, "seven")]);

    let entries = testing::flatten(&map).unwrap();
    assert_eq!(
        entries,
        vec![("7".into(), ser::FieldValue::Str("seven".into()))]
    );
}

/// An array of scalars is joined into one string.
#[test]
fn scalar_array() {
    #[derive(Serialize)]
    struct Props {
        lines: Vec<&'static str>,
    }

    let entries = testing::flatten(&Props {
        lines: vec!["JY", "JC"],
    })
    .unwrap();
    assert_eq!(
        entries,
        vec![("lines".into(), ser::FieldValue::Str("JY,JC".into()))]
    );
}

/// A tuple of scalars is joined into one string as well.
#[test]
fn tuple() {
    #[derive(Serialize)]
    struct Props {
        pair: (i32, &'static str),
    }

    let entries = testing::flatten(&Props { pair: (1, "a") }).unwrap();
    assert_eq!(
        entries,
        vec![("pair".into(), ser::FieldValue::Str("1,a".into()))]
    );
}

/// An array of structs has a key per element.
#[test]
fn struct_array() {
    #[derive(Serialize)]
    struct Props {
        exits: Vec<Exit>,
    }
    #[derive(Serialize)]
    struct Exit {
        name: &'static str,
    }

    let entries = testing::flatten(&Props {
        exits: vec![Exit { name: "North" }, Exit { name: "South" }],
    })
    .unwrap();
    assert_eq!(
        entries,
        vec![
            ("exits[0].name".into(), ser::FieldValue::Str("North".into())),
            ("exits[1].name".into(), ser::FieldValue::Str("South".into())),
        ]
    );
}

/// An array of arrays has a key per element, each joined into one string.
#[test]
fn nested_array() {
    #[derive(Serialize)]
    struct Props {
        rows: Vec<Vec<i32>>,
    }

    let entries = testing::flatten(&Props {
        rows: vec![vec![1, 2], vec![3]],
    })
    .unwrap();
    assert_eq!(
        entries,
        vec![
            ("rows[0]".into(), ser::FieldValue::Str("1,2".into())),
            ("rows[1]".into(), ser::FieldValue::Str("3".into())),
        ]
    );
}

/// A unit variant is written as its name under the field name.
#[test]
fn unit_variant() {
    #[derive(Serialize)]
    struct Props {
        kind: Kind,
    }
    #[derive(Serialize)]
    enum Kind {
        Terminal,
    }

    let entries = testing::flatten(&Props {
        kind: Kind::Terminal,
    })
    .unwrap();
    assert_eq!(
        entries,
        vec![("kind".into(), ser::FieldValue::Str("Terminal".into()))]
    );
}

/// A newtype variant has its value under the variant name.
#[test]
fn newtype_variant() {
    #[derive(Serialize)]
    struct Props {
        shape: Shape,
    }
    #[derive(Serialize)]
    enum Shape {
        Circle(f64),
    }

    let entries = testing::flatten(&Props {
        shape: Shape::Circle(1.5),
    })
    .unwrap();
    assert_eq!(
        entries,
        vec![("shape.Circle".into(), ser::FieldValue::F64(1.5))]
    );
}

/// A struct variant has its fields under the variant name.
#[test]
fn struct_variant() {
    #[derive(Serialize)]
    struct Props {
        shape: Shape,
    }
    #[derive(Serialize)]
    enum Shape {
        Rect { width: f64 },
    }

    let entries = testing::flatten(&Props {
        shape: Shape::Rect { width: 2.0 },
    })
    .unwrap();
    assert_eq!(
        entries,
        vec![("shape.Rect.width".into(), ser::FieldValue::F64(2.0))]
    );
}

/// `Some` is written as its value.
#[test]
fn some() {
    #[derive(Serialize)]
    struct Props {
        capacity: Option<u32>,
    }

    let entries = testing::flatten(&Props {
        capacity: Some(100),
    })
    .unwrap();
    assert_eq!(
        entries,
        vec![("capacity".into(), ser::FieldValue::U32(100))]
    );
}

/// `None`, `()` and an empty array write no property.
#[test]
fn empty_values_skipped() {
    #[derive(Serialize)]
    struct Props {
        name: &'static str,
        capacity: Option<u32>,
        unit: (),
        lines: Vec<&'static str>,
    }

    let entries = testing::flatten(&Props {
        name: "Tokyo",
        capacity: None,
        unit: (),
        lines: vec![],
    })
    .unwrap();
    assert_eq!(
        entries,
        vec![("name".into(), ser::FieldValue::Str("Tokyo".into()))]
    );
}
