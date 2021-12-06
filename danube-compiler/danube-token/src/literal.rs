use crate::TokenKind;

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Boolean(bool),
    Integer(i64),
    Float(f64),
    Char(char),
    String(String),
}

impl From<Literal> for TokenKind {
    fn from(literal: Literal) -> Self {
        TokenKind::Literal(literal)
    }
}

impl From<bool> for Literal {
    #[inline(always)]
    fn from(value: bool) -> Self {
        Literal::Boolean(value)
    }
}

impl From<i64> for Literal {
    #[inline(always)]
    fn from(value: i64) -> Self {
        Literal::Integer(value)
    }
}

impl From<f64> for Literal {
    #[inline(always)]
    fn from(value: f64) -> Self {
        Literal::Float(value)
    }
}

impl From<char> for Literal {
    #[inline(always)]
    fn from(value: char) -> Self {
        Literal::Char(value)
    }
}

impl From<String> for Literal {
    #[inline(always)]
    fn from(value: String) -> Self {
        Literal::String(value)
    }
}
