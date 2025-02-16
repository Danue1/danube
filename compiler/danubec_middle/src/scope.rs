use danubec_hir::{DefId, Ident};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Environment {
    scopes: Vec<Scope>,
}

#[derive(Debug)]
pub struct Scope {
    types: Rib,
    variables: Rib,
}

#[derive(Debug)]
pub struct Rib {
    kind: RibKind,
    map: HashMap<Ident, DefId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RibKind {
    Module,
    Function,
    Struct,
    Enum,
    Trait,
    Impl,
    Block,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NamespaceKind {
    Type,
    Variable,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            scopes: vec![Scope::new(RibKind::Module)],
        }
    }

    #[inline]
    pub fn push(&mut self, kind: RibKind) {
        self.scopes.push(Scope::new(kind));
    }

    #[inline]
    pub fn pop(&mut self) {
        self.scopes.pop();
    }

    pub fn define(&mut self, namespace: NamespaceKind, ident: Ident, def_id: DefId) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.define(namespace, ident, def_id);
        }
    }

    pub fn resolve(&self, namespace: NamespaceKind, ident: Ident) -> Option<DefId> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.resolve(namespace, ident))
    }
}

impl Scope {
    #[inline]
    fn new(kind: RibKind) -> Self {
        Self {
            types: Rib::new(kind),
            variables: Rib::new(kind),
        }
    }

    #[inline]
    fn define(&mut self, kind: NamespaceKind, ident: Ident, def_id: DefId) {
        self[kind].define(ident, def_id);
    }

    #[inline]
    fn resolve(&self, kind: NamespaceKind, ident: Ident) -> Option<DefId> {
        self[kind].resolve(&ident)
    }
}

impl std::ops::Index<NamespaceKind> for Scope {
    type Output = Rib;

    #[inline]
    fn index(&self, namespace: NamespaceKind) -> &Self::Output {
        match namespace {
            NamespaceKind::Type => &self.types,
            NamespaceKind::Variable => &self.variables,
        }
    }
}

impl std::ops::IndexMut<NamespaceKind> for Scope {
    #[inline]
    fn index_mut(&mut self, namespace: NamespaceKind) -> &mut Self::Output {
        match namespace {
            NamespaceKind::Type => &mut self.types,
            NamespaceKind::Variable => &mut self.variables,
        }
    }
}

impl Rib {
    #[inline]
    fn new(kind: RibKind) -> Self {
        Self {
            kind,
            map: HashMap::new(),
        }
    }

    #[inline]
    pub const fn kind(&self) -> RibKind {
        self.kind
    }

    #[inline]
    fn define(&mut self, ident: Ident, def_id: DefId) {
        self.map.insert(ident, def_id);
    }

    #[inline]
    fn resolve(&self, ident: &Ident) -> Option<DefId> {
        self.map.get(ident).copied()
    }
}
