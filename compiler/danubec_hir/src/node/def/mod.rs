pub mod module_def;
pub mod struct_def;
pub mod use_def;

pub use module_def::*;
pub use struct_def::*;
pub use use_def::*;

#[derive(Debug)]
pub struct Definition {
    pub kind: DefinitionKind,
}

#[derive(Debug)]
pub enum DefinitionKind {
    // Const(ConstDef),
    // Enum(EnumDef),
    // Function(FunctionDef),
    // Impl(ImplDef),
    Module(ModuleDef),
    // Static(StaticDef),
    Struct(StructDef),
    // Trait(TraitDef),
    // Type(TypeDef),
    Use(UseDef),
}
