pub mod module_def;
pub mod struct_def;

pub use module_def::*;
pub use struct_def::*;

use super::Ident;

#[derive(Debug)]
pub struct Definition {
    pub name: Ident,
    pub kind: DefinitionKind,
}

#[derive(Debug)]
pub enum DefinitionKind {
    Module(ModuleDef),
    // Function(FunctionDef),
    Struct(StructDef),
    // Enum(EnumDef),
    // Const(ConstDef),
    // Static(StaticDef),
    // Trait(TraitDef),
    // Type(TypeDef),
    // Impl(ImplDef),
}
