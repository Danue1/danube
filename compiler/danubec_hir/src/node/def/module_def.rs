use crate::{DefId, Ident};

#[derive(Debug)]
pub struct ModuleDef {
    pub ident: Ident,
    pub definitions: Vec<DefId>,
}
