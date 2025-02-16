use super::Ident;
use crate::{KrateId, ModDefId};
use std::collections::HashMap;

pub struct Krate {
    pub modules: Vec<ModDefId>,
    pub children: HashMap<Ident, KrateId>,
}
