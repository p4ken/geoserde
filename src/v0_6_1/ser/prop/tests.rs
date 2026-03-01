#![cfg(test)]

use serde::Serialize;

use crate::v0_6_1::ser::prop::FlatProperties;

#[test]
fn flatten_struct() {
    #[derive(Serialize)]
    struct Root {
        parent: Parent,
        child: Child,
    }

    #[derive(Serialize)]
    struct Parent {
        child: Child,
    }

    #[derive(Serialize)]
    struct Child {
        text: &'static str,
    }

    let root = Root {
        parent: Parent {
            child: Child { text: "hello" },
        },
        child: Child { text: "world" },
    };

    let mut buf = Vec::new();
    let mut json_ser = serde_json::Serializer::new(&mut buf);
    let ser = FlatProperties::from_serializer(&mut json_ser);
    root.serialize(ser).unwrap();
    assert_eq!(
        r#"{"parent.child.text":"hello","child.text":"world"}"#,
        String::from_utf8(buf).unwrap()
    );
}

#[test]
fn flatten_map() {
    let root = serde_json::json!({"parent":{"child":{"text":"hello"}},"child":{"text":"world"}});

    let mut buf = Vec::new();
    let mut json_ser = serde_json::Serializer::new(&mut buf);
    let ser = FlatProperties::from_serializer(&mut json_ser);
    root.serialize(ser).unwrap();
    assert_eq!(
        r#"{"parent.child.text":"hello","child.text":"world"}"#,
        String::from_utf8(buf).unwrap()
    );
}

#[test]
fn flatten_struct_seq() {
    #[derive(Serialize)]
    struct Root {
        parent: Vec<Parent>,
    }

    #[derive(Serialize)]
    struct Parent {
        child: Vec<Child>,
    }

    #[derive(Serialize)]
    struct Child {
        text: &'static str,
    }

    let root = Root {
        parent: vec![
            Parent {
                child: vec![
                    Child { text: "one" },
                    Child { text: "two" },
                    Child { text: "three" },
                ],
            },
            Parent {
                child: vec![Child { text: "another" }],
            },
        ],
    };

    let mut buf = Vec::new();
    let mut json_ser = serde_json::Serializer::new(&mut buf);
    let ser = FlatProperties::from_serializer(&mut json_ser);
    root.serialize(ser).unwrap();
    assert_eq!(
        r#"{"parent[0].child[0].text":"one","parent[0].child[1].text":"two","parent[0].child[2].text":"three","parent[1].child[0].text":"another"}"#,
        String::from_utf8(buf).unwrap()
    );
}

#[test]
fn flatten_value_seq() {
    #[derive(Serialize)]
    struct Root {
        child: Vec<Child>,
    }

    #[derive(Serialize)]
    struct Child {
        text: Vec<&'static str>,
    }

    let root = Root {
        child: vec![Child {
            text: vec!["one", "two", "three"],
        }],
    };

    let mut buf = Vec::new();
    let mut json_ser = serde_json::Serializer::new(&mut buf);
    let ser = FlatProperties::from_serializer(&mut json_ser);
    root.serialize(ser).unwrap();
    assert_eq!(
        r#"{"child[0].text":"one,two,three"}"#,
        String::from_utf8(buf).unwrap()
    );
}
