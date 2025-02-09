pub mod function_definition;

pub use function_definition::*;

ast_node! {
    struct Definition;

    node visibility -> Visibility;
    node kind -> DefinitionKind;
}

ast_node! {
    enum DefinitionKind {
        Function(FunctionDefinition),
    }
}
