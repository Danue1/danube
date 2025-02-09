pub mod function_definition;
pub mod type_definition;

pub use function_definition::*;
pub use type_definition::*;

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
    enum DefinitionKind {
        Function(FunctionDefinition),
        Type(TypeDefinition),
    }
}
