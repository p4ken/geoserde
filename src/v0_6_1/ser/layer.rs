use std::borrow::Cow;

use serde::Serialize;

pub struct OrderedColumns {
    _keys: Vec<Column>,
}

impl OrderedColumns {
    pub fn new() -> Self {
        Self { _keys: Vec::new() }
    }

    pub fn merge(&mut self, properties: impl Serialize) {}
}

struct Column {
    name: Cow<'static, str>,
    col_type: super::FieldValue<'static>,
}
