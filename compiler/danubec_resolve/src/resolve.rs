mod definition;

use crate::{Resolved, collect::Collector};
use danubec_data_structure::Arena;
use danubec_middle::{ast, hir};
use danubec_symbol::Symbol;
use std::collections::HashMap;

pub struct Resolver {
    pub(crate) collector: Collector,
    pub(crate) definitions: HashMap<hir::DefId, hir::Definition>,
    pub(crate) bodies: Arena<hir::BodyId, hir::Body>,
}

impl Resolver {
    pub fn new(collector: Collector) -> Self {
        Self {
            collector,
            definitions: HashMap::new(),
            bodies: Arena::new(),
        }
    }

    #[inline]
    pub fn resolve(&mut self, name: Symbol, krate: &ast::Krate) {
        self.resolve_krate(name, krate);
    }

    pub fn finalize(self) -> Resolved {
        Resolved {
            definitions: self.definitions,
            bodies: self.bodies,
        }
    }
}
