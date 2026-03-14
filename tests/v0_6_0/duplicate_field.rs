struct Parent {
    foo: i32,
    child: Child,
}

struct Child {
    foo: String,
}

#[test]
fn duplicate_field() {
    let obj = Parent {
        foo: 123,
        child: Child { foo: "ABC".into() },
    };
}
