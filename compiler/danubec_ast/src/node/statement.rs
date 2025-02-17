use super::{Definition, Expression, Pattern, Type};

pub struct Statement {
    pub kind: StatementKind,
}

pub enum StatementKind {
    Definition(Definition),
    Expression(Expression),
    Let {
        pattern: Pattern,
        ty: Option<Type>,
        expression: Option<Expression>,
    },
    Semicolon,
}
