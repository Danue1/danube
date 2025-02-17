use super::Visibility;
use danubec_syntax::SyntaxNode;

pub struct Definition {
    pub syntax: SyntaxNode,
    pub visibility: Visibility,
    pub kind: DefinitionKind,
}

pub enum DefinitionKind {
    // Const(ConstDef),
    // Enum(EnumDef),
    // Function(FunctionDef),
    // Impl(ImplDef),
    // Module(ModuleDef),
    // Static(StaticDef),
    // Struct(StructDef),
    // Trait(TraitDef),
    // Type(TypeDef),
    // Use(UseDef),
}
