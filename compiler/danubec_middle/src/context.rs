use crate::hir;
use danubec_diagnostic::Diagnostic;
use danubec_symbol::Symbol;
use std::{collections::HashMap, path::PathBuf};

#[derive(Clone, Copy)]
pub enum EntryKind {
    Lib,
    Main,
}

pub struct Context {
    pub working_directory: PathBuf,

    pub ident_to_krate: HashMap<Symbol, hir::KrateId>,
    pub krates: HashMap<hir::KrateId, hir::Krate>,
    pub definitions: HashMap<hir::DefId, hir::Definition>,
    pub bodies: HashMap<hir::BodyId, hir::Body>,

    pub diagnostic: Diagnostic,
}

impl Context {
    pub fn new(working_directory: PathBuf) -> Self {
        Context {
            working_directory,

            ident_to_krate: HashMap::new(),
            krates: HashMap::new(),
            definitions: HashMap::new(),
            bodies: HashMap::new(),

            diagnostic: Diagnostic::new(),
        }
    }
}
