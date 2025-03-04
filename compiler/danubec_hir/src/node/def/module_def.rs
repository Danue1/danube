use crate::HirId;
use danubec_symbol::Symbol;

#[derive(Debug)]
pub struct ModuleDef {
    pub ident: Symbol,
    pub definitions: Vec<HirId>,
}
