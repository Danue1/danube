use danubec_hir::{DefId, Ident};
use std::collections::HashMap;

pub struct Scope {
    definitions: RibStack,
    variables: RibStack,
}

pub struct RibStack {
    stack: Vec<Rib>,
}

pub struct Rib {
    pub kind: RibKind,

    map: HashMap<Ident, DefId>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RibKind {
    Module,
    Constant,
    Function,
    /// The definitions in `impl trait`, `trait`.
    Associative,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Namespace {
    Definition,
    Variable,
}

impl Scope {
    pub fn new() -> Self {
        Scope {
            definitions: RibStack::new_kind(RibKind::Module),
            variables: RibStack::new(),
        }
    }
}

impl RibStack {
    pub const fn new() -> Self {
        RibStack { stack: vec![] }
    }

    pub fn new_kind(kind: RibKind) -> Self {
        RibStack {
            stack: vec![Rib::new(kind)],
        }
    }

    #[inline]
    pub fn push(&mut self, rib: Rib) {
        self.stack.push(rib);
    }

    #[inline]
    pub fn pop(&mut self) {
        self.stack.pop();
    }

    pub fn define(&mut self, ident: Ident, def_id: DefId) {
        if let Some(rib) = self.stack.last_mut() {
            rib.define(ident, def_id);
        }
    }

    pub fn resolve(&self, ident: Ident) -> Option<DefId> {
        for rib in self.stack.iter().rev() {
            if let Some(def_id) = rib.resolve(ident) {
                return Some(def_id);
            }
        }

        None
    }
}

impl Rib {
    #[inline]
    pub fn new(kind: RibKind) -> Self {
        Rib {
            kind,
            map: HashMap::new(),
        }
    }

    #[inline]
    pub fn define(&mut self, ident: Ident, def_id: DefId) {
        self.map.insert(ident, def_id);
    }

    #[inline]
    pub fn resolve(&self, ident: Ident) -> Option<DefId> {
        self.map.get(&ident).copied()
    }
}

impl std::ops::Index<Namespace> for Scope {
    type Output = RibStack;

    #[inline]
    fn index(&self, namespace: Namespace) -> &Self::Output {
        match namespace {
            Namespace::Definition => &self.definitions,
            Namespace::Variable => &self.variables,
        }
    }
}

impl std::ops::IndexMut<Namespace> for Scope {
    #[inline]
    fn index_mut(&mut self, namespace: Namespace) -> &mut Self::Output {
        match namespace {
            Namespace::Definition => &mut self.definitions,
            Namespace::Variable => &mut self.variables,
        }
    }
}
