pub mod namespace;
pub mod next_id;
pub mod scope;
pub mod symbol;

pub use namespace::*;
pub use next_id::*;
pub use scope::*;
pub use symbol::*;

use crate::node::{Body, BodyId};
use arena::Arena;
use std::collections::HashMap;

pub struct Context {
    namespaces: HashMap<String, Namespace>,
    scopes: Vec<Scope>,
    bodies: Arena<BodyId, Body>,
    next_id: NextId,
}

impl Context {
    pub fn new() -> Self {
        Context {
            namespaces: HashMap::new(),
            scopes: vec![Scope::new()],
            bodies: Arena::new(),
            next_id: NextId::new(),
        }
    }

    #[inline]
    pub fn next_id(&mut self) -> Symbol {
        self.next_id.next()
    }

    #[inline]
    pub fn add_namespace(&mut self, name: &str, namespace: Namespace) {
        self.namespaces.insert(name.to_owned(), namespace);
    }

    #[inline]
    pub fn add_body(&mut self, body: Body) -> BodyId {
        self.bodies.alloc(body)
    }

    pub fn find_namespace_by_path(&self, path: &[&str]) -> Option<&Namespace> {
        let (&head, tail) = path.split_first()?;
        let mut namespace = self.namespaces.get(head)?;
        for name in tail {
            namespace = namespace.get(name)?;
        }

        Some(namespace)
    }

    #[inline]
    pub fn with_scope<F>(&mut self, f: F)
    where
        F: FnOnce(&mut Self),
    {
        self.scopes.push(Scope::new());
        f(self);
        self.scopes.pop();
    }
}
