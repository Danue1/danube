pub mod comment;
pub mod identifier;
pub mod keyword;
pub mod literal;
pub mod span;
pub mod symbol;

pub use comment::*;
pub use identifier::*;
pub use keyword::*;
pub use literal::*;
pub use span::*;
pub use symbol::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub span: Span,
    pub kind: TokenKind,
}

impl Token {
    pub const fn new(span: Span, kind: TokenKind) -> Self {
        Token { span, kind }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Identifier(Identifier),
    Keyword(Keyword),
    Symbol(Symbol),
    Literal(Literal),
    Comment(Comment),
}
