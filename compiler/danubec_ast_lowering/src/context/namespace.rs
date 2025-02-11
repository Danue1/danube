use super::Scope;
use std::collections::HashMap;

pub struct Namespace {
    scope: Scope,
    children: HashMap<String, Namespace>,
}

impl Namespace {
    pub fn new() -> Self {
        Self::from_raw(Scope::new(), HashMap::new())
    }

    pub const fn from_raw(scope: Scope, children: HashMap<String, Namespace>) -> Self {
        Namespace { scope, children }
    }

    #[inline]
    pub fn get(&self, name: &str) -> Option<&Namespace> {
        self.children.get(name)
    }
}
