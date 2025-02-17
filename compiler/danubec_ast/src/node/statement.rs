use super::{Definition, Expression, Path, Type};
use danubec_syntax::SyntaxNode;

pub struct Statement {
    pub syntax: SyntaxNode,
    pub kind: StatementKind,
}

pub enum StatementKind {
    Definition(Definition),
    Expression(Expression),
    Let {
        pattern: Path,
        ty: Option<Type>,
        value: Option<Expression>,
    },
    Semicolon,
}
