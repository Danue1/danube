use crate::{KrateId, ModDefId};
use danubec_symbol::Symbol;
use std::collections::HashMap;

pub struct Krate {
    pub modules: Vec<ModDefId>,
    pub children: HashMap<Symbol, KrateId>,
}
