#![warn(clippy::all)]

mod collect;
mod resolve;

use collect::*;
use resolve::*;

use danubec_data_structure::Arena;
use danubec_middle::{ast, hir};
use danubec_symbol::Symbol;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Resolved {
    pub definitions: HashMap<hir::DefId, hir::Definition>,
    pub bodies: Arena<hir::BodyId, hir::Body>,
}

pub fn resolve(name: Symbol, crates: HashMap<Symbol, ast::Krate>) -> Resolved {
    let mut collector = Collector::new();
    for (&name, krate) in &crates {
        collector.collect_krate(name, krate);
    }

    let mut resolver = Resolver::new(collector);
    if let Some(krate) = crates.get(&name) {
        resolver.resolve(name, krate);
    }

    resolver.finalize()
}
