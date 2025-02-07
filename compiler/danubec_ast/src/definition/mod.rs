pub mod function_definition;

pub use function_definition::*;

crate::ast_node! {
    enum Definition {
        Function(FunctionDefinition),
    }
}
