pub mod function_definition;

pub use function_definition::*;

ast_node! {
    enum Definition {
        Function(FunctionDefinition),
    }
}
