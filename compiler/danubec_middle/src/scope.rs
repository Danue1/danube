use danubec_hir::HirId;
use danubec_symbol::Symbol;
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
    map: HashMap<Symbol, HirId>,
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

    pub fn define(&mut self, namespace: NamespaceKind, name: Symbol, def_id: HirId) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.define(namespace, name, def_id);
        }
    }

    pub fn resolve(&self, namespace: NamespaceKind, name: Symbol) -> Option<HirId> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.resolve(namespace, name))
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
    fn define(&mut self, kind: NamespaceKind, name: Symbol, def_id: HirId) {
        self[kind].define(name, def_id);
    }

    #[inline]
    fn resolve(&self, kind: NamespaceKind, name: Symbol) -> Option<HirId> {
        self[kind].resolve(&name)
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
    fn define(&mut self, name: Symbol, def_id: HirId) {
        self.map.insert(name, def_id);
    }

    #[inline]
    fn resolve(&self, name: &Symbol) -> Option<HirId> {
        self.map.get(name).copied()
    }
}
