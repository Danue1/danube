pub mod scope;

pub use scope::*;

use crate::hir;
use danubec_diagnostic::Diagnostic;
use std::collections::HashMap;

pub struct Context {
    pub working_directory: String,

    pub krates: HashMap<hir::KrateId, hir::Krate>,
    pub definitions: HashMap<hir::DefId, hir::Definition>,
    pub bodies: HashMap<hir::BodyId, hir::Body>,

    pub ident_to_krate: HashMap<hir::Ident, hir::KrateId>,
    pub scope: Scope,

    pub diagnostic: Diagnostic,
}

impl Context {
    pub fn new(working_directory: String) -> Self {
        Context {
            working_directory: working_directory.into(),

            krates: HashMap::new(),
            definitions: HashMap::new(),
            bodies: HashMap::new(),

            ident_to_krate: HashMap::new(),
            scope: Scope::new(),

            diagnostic: Diagnostic::new(),
        }
    }

    pub fn with_rib<F, T>(&mut self, namespace: Namespace, kind: RibKind, f: F) -> T
    where
        F: FnOnce(&mut Self) -> T,
    {
        self.scope[namespace].push(Rib::new(kind));
        let result = f(self);
        self.scope[namespace].pop();

        result
    }

    pub fn with_scope<F, T>(&mut self, kind: RibKind, f: F) -> T
    where
        F: FnOnce(&mut Self) -> T,
    {
        self.with_rib(Namespace::Definition, kind, |context| {
            context.with_rib(Namespace::Variable, kind, f)
        })
    }
}
