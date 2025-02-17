pub mod assignment_expression;
pub mod binary_expression;
pub mod block_expression;
pub mod let_expression;
pub mod literal_expression;
pub mod unary_expression;

pub use assignment_expression::*;
pub use binary_expression::*;
pub use block_expression::*;
pub use let_expression::*;
pub use literal_expression::*;
pub use unary_expression::*;

ast_node! {
    /// ```ebnf
    /// Expression =
    /// | AssignmentKind
    /// ```
    struct Expression;

    node kind -> ExpressionKind;
}

ast_node! {
    /// ```ebnf
    /// ExpressionKind =
    /// | AssignmentExpression
    /// | BinaryExpression
    /// | BlockExpression
    /// | LetExpression
    /// | LiteralExpression
    /// | UnaryExpression
    /// ```
    enum ExpressionKind {
        Assignment(AssignmentExpression),
        Binary(BinaryExpression),
        Block(BlockExpression),
        Let(LetExpression),
        Literal(LiteralExpression),
        Unary(UnaryExpression),
    }
}
