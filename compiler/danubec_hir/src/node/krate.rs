use crate::{HirId, KrateId};
use danubec_symbol::Symbol;
use std::collections::HashMap;

pub struct Krate {
    pub modules: Vec<HirId>,
    pub children: HashMap<Symbol, KrateId>,
}
