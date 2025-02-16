use crate::DefId;

#[derive(Debug)]
pub struct ModuleDef {
    pub definitions: Vec<DefId>,
}
