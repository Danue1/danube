use crate::Literal;

#[derive(Debug, PartialEq)]
pub struct Expression {
    pub kind: ExpressionKind,
}

#[derive(Debug, PartialEq)]
pub enum ExpressionKind {
    Literal(Literal),
}

impl Expression {
    pub const fn new(kind: ExpressionKind) -> Self {
        Self { kind }
    }
}
