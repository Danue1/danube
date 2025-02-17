use super::{Definition, Expression, Type};
use danubec_symbol::Symbol;

pub struct Statement {
    pub kind: StatementKind,
}

pub enum StatementKind {
    Definition(Definition),
    Expression(Expression),
    Let {
        pattern: Symbol,
        ty: Option<Type>,
        expression: Option<Expression>,
    },
    Semicolon,
}
