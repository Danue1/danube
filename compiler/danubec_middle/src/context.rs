use crate::hir;
use danubec_diagnostic::Diagnostic;
use std::collections::HashMap;

pub struct Context {
    pub working_directory: String,

    pub ident_to_krate: HashMap<hir::Ident, hir::KrateId>,
    pub krates: HashMap<hir::KrateId, hir::Krate>,
    pub definitions: HashMap<hir::DefId, hir::Definition>,
    pub bodies: HashMap<hir::BodyId, hir::Body>,

    pub diagnostic: Diagnostic,
}

impl Context {
    pub fn new(working_directory: String) -> Self {
        Context {
            working_directory: working_directory.into(),

            ident_to_krate: HashMap::new(),
            krates: HashMap::new(),
            definitions: HashMap::new(),
            bodies: HashMap::new(),

            diagnostic: Diagnostic::new(),
        }
    }
}
