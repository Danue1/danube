use crate::hir;
use danubec_diagnostic::Diagnostic;
use std::collections::HashMap;

pub struct Context {
    pub working_directory: String,
    pub crates: HashMap<hir::CrateId, hir::Krate>,
    pub diagnostic: Diagnostic,
}

impl Context {
    pub fn new(working_directory: String) -> Self {
        Context {
            working_directory: working_directory.into(),
            crates: HashMap::new(),
            diagnostic: Diagnostic::new(),
        }
    }
}
