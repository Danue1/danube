use crate::{Position, TokenKind};

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub position: Position,
    pub kind: TokenKind,
}
