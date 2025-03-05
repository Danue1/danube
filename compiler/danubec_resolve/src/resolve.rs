mod definition;
mod path;
mod r#type;

use crate::{Resolved, collect::Collector};
use danubec_data_structure::Arena;
use danubec_middle::hir;
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
    pub fn add_definition(&mut self, def_id: hir::DefId, definition: hir::Definition) {
        self.definitions.insert(def_id, definition);
    }

    #[inline]
    pub fn add_body(&mut self, body: hir::Body) -> hir::BodyId {
        self.bodies.alloc(body)
    }

    pub fn finalize(self) -> Resolved {
        Resolved {
            definitions: self.definitions,
            bodies: self.bodies,
        }
    }
}
