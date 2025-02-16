pub mod module_def;

pub use module_def::*;

use super::Ident;

pub struct Definition {
    pub name: Ident,
    pub kind: DefinitionKind,
}

pub enum DefinitionKind {
    Module(ModuleDef),
    // Function(FunctionDef),
    // Struct(StructDef),
    // Enum(EnumDef),
    // Const(ConstDef),
    // Static(StaticDef),
    // Trait(TraitDef),
    // Type(TypeDef),
    // Impl(ImplDef),
}
