pub mod definition_statement;
pub mod expression_statement;
pub mod let_statement;
pub mod semicolon_statement;

pub use definition_statement::*;
pub use expression_statement::*;
pub use let_statement::*;
pub use semicolon_statement::*;

ast_node! {
    /// ```ebnf
    /// Statement =
    /// | StatementKind
    /// ```
    struct Statement;

    node kind -> StatementKind;
}

ast_node! {
    /// ```ebnf
    /// StatementKind =
    /// | DefinitionStatement
    /// | ExpressionStatement
    /// | LetStatement
    /// | SemicolonStatement
    /// ```
    enum StatementKind {
        Definition(DefinitionStatement),
        Expression(ExpressionStatement),
        Let(LetStatement),
        Semicolon(SemicolonStatement),
    }
}
