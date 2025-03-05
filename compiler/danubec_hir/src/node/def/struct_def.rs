use super::{Predicate, TypeParameter};
use crate::{DefId, HirId, Type};
use danubec_symbol::Symbol;

#[derive(Debug)]
pub struct StructDef {
    pub def_id: DefId,
    pub ident: Symbol,
    pub type_parameters: Vec<TypeParameter>,
    pub predicates: Vec<Predicate>,
    pub kind: Option<StructKind>,
}

#[derive(Debug)]
pub enum StructKind {
    Named { fields: Vec<(HirId, Symbol, Type)> },
    Unnamed { fields: Vec<Type> },
}
