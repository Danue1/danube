pub mod assignment_expression;
pub mod block_expression;
pub mod let_expression;
pub mod literal_expression;

pub use assignment_expression::*;
pub use block_expression::*;
pub use let_expression::*;
pub use literal_expression::*;

crate::ast_node! {
    enum Expression {
        Assignment(AssignmentExpression),
        Block(BlockExpression),
        Let(LetExpression),
        Literal(LiteralExpression),
    }
}
