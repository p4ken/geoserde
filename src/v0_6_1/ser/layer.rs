use std::borrow::Cow;

pub struct PreOrderedKeys {
    _keys: Vec<Cow<'static, str>>,
}

impl PreOrderedKeys {
    pub fn new() -> Self {
        Self { _keys: Vec::new() }
    }
}
