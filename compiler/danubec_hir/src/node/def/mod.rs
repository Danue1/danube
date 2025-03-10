pub mod module_def;
pub mod struct_def;
pub mod use_def;

pub use module_def::*;
pub use struct_def::*;
pub use use_def::*;

use super::Type;
use crate::HirId;
use danubec_symbol::Symbol;

#[derive(Debug)]
pub struct Definition {
    pub kind: DefinitionKind,
}

#[derive(Debug)]
pub enum DefinitionKind {
    Module {
        ident: Symbol,
        definitions: Vec<HirId>,
    },
    Use(UseDef),
    Struct(StructDef),
    // Const(ConstDef),
    // Enum(EnumDef),
    // Function(FunctionDef),
    // Impl(ImplDef),
    // Static(StaticDef),
    // Trait(TraitDef),
    // Type(TypeDef),
}

#[derive(Debug)]
pub struct TypeParameter {
    pub ty: Type,
    pub bounds: Vec<Type>,
}

#[derive(Debug)]
pub struct Predicate {
    pub ty: Type,
    pub bounds: Vec<Type>,
}
