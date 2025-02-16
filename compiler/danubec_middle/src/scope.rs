use danubec_hir::{DefId, Ident};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Environment {
    scopes: Vec<Scope>,
}

#[derive(Debug)]
pub struct Scope {
    kind: ScopeKind,
    types: HashMap<Ident, DefId>,
    variables: HashMap<Ident, DefId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScopeKind {
    Module,
    Function,
    Struct,
    Enum,
    Trait,
    Impl,
    Block,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Namespace {
    Type,
    Variable,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            scopes: vec![Scope::new(ScopeKind::Module)],
        }
    }

    #[inline]
    pub fn push(&mut self, kind: ScopeKind) {
        self.scopes.push(Scope::new(kind));
    }

    #[inline]
    pub fn pop(&mut self) {
        self.scopes.pop();
    }

    pub fn define(&mut self, namespace: Namespace, ident: Ident, def_id: DefId) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.define(namespace, ident, def_id);
        }
    }

    pub fn resolve(&self, namespace: Namespace, ident: Ident) -> Option<DefId> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.resolve(namespace, ident))
    }
}

impl Scope {
    #[inline]
    fn new(kind: ScopeKind) -> Self {
        Self {
            kind,
            types: HashMap::new(),
            variables: HashMap::new(),
        }
    }

    #[inline]
    pub const fn kind(&self) -> ScopeKind {
        self.kind
    }

    #[inline]
    fn define(&mut self, namespace: Namespace, ident: Ident, def_id: DefId) {
        self[namespace].insert(ident, def_id);
    }

    #[inline]
    fn resolve(&self, namespace: Namespace, ident: Ident) -> Option<DefId> {
        self[namespace].get(&ident).copied()
    }
}

impl std::ops::Index<Namespace> for Scope {
    type Output = HashMap<Ident, DefId>;

    #[inline]
    fn index(&self, namespace: Namespace) -> &Self::Output {
        match namespace {
            Namespace::Type => &self.types,
            Namespace::Variable => &self.variables,
        }
    }
}

impl std::ops::IndexMut<Namespace> for Scope {
    #[inline]
    fn index_mut(&mut self, namespace: Namespace) -> &mut Self::Output {
        match namespace {
            Namespace::Type => &mut self.types,
            Namespace::Variable => &mut self.variables,
        }
    }
}
