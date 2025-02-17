pub mod const_definition;
pub mod enum_definition;
pub mod function_definition;
pub mod impl_definition;
pub mod module_definition;
pub mod static_definition;
pub mod struct_definition;
pub mod trait_definition;
pub mod type_definition;
pub mod use_definition;

pub use const_definition::*;
pub use enum_definition::*;
pub use function_definition::*;
pub use impl_definition::*;
pub use module_definition::*;
pub use static_definition::*;
pub use struct_definition::*;
pub use trait_definition::*;
pub use type_definition::*;
pub use use_definition::*;

ast_node! {
    /// ```ebnf
    /// Definition =
    /// | DefinitionKind
    /// | Visibility _ DefinitionKind
    /// ```
    struct Definition;

    node visibility -> Visibility;
    node kind -> DefinitionKind;
}

ast_node! {
    /// ```ebnf
    /// DefinitionKind =
    /// | ConstDefinition
    /// | EnumDefinition
    /// | FunctionDefinition
    /// | ImplDefinition
    /// | ModuleDefinition
    /// | StaticDefinition
    /// | StructDefinition
    /// | TraitDefinition
    /// | TypeDefinition
    /// | UseDefinition
    /// ```
    enum DefinitionKind {
        Const(ConstDefinition),
        Enum(EnumDefinition),
        Function(FunctionDefinition),
        Impl(ImplDefinition),
        Module(ModuleDefinition),
        Static(StaticDefinition),
        Struct(StructDefinition),
        Trait(TraitDefinition),
        Type(TypeDefinition),
        Use(UseDefinition),
    }
}
