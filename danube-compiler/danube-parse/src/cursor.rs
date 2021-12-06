use danube_token::Token;
use std::iter::Peekable;
use std::slice::Iter;

#[derive(Debug, Clone)]
pub struct Cursor<'parse> {
    cursor: usize,
    iter: Peekable<Iter<'parse, Token>>,
}

impl<'parse> Cursor<'parse> {
    #[inline]
    pub fn new(tokens: &'parse [Token]) -> Self {
        Cursor {
            cursor: 0,
            iter: tokens.iter().peekable(),
        }
    }

    #[inline(always)]
    pub fn cursor(&self) -> usize {
        self.cursor
    }

    #[inline]
    pub fn peek(&mut self) -> Option<&Token> {
        match self.iter.peek() {
            Some(token) => Some(token),
            None => None,
        }
    }
}

impl<'parse> std::iter::Iterator for Cursor<'parse> {
    type Item = &'parse Token;

    #[inline]
    fn next(&mut self) -> Option<&'parse Token> {
        self.cursor += 1;
        self.iter.next()
    }
}

#[macro_export]
macro_rules! identifier {
    ($cursor:expr) => {{
        use danube_token::{Token, TokenKind};

        match $cursor.peek() {
            Some(Token {
                span: _,
                kind: TokenKind::Identifier(identifier),
            }) => Some(identifier),
            _ => None,
        }
    }};
}

#[macro_export]
macro_rules! keyword {
    ($cursor:expr) => {{
        use danube_token::{Token, TokenKind};

        match $cursor.peek() {
            Some(Token {
                span: _,
                kind: TokenKind::Keyword(keyword),
            }) => Some(keyword),
            _ => None,
        }
    }};
    ($cursor:expr => $ident:ident) => {{
        use danube_token::{Keyword, Token, TokenKind};

        match $cursor.peek() {
            Some(Token {
                span: _,
                kind: TokenKind::Keyword(Keyword::$ident),
            }) => {
                $cursor.next();
                true
            }
            _ => false,
        }
    }};
}

#[macro_export]
macro_rules! symbol {
    ($cursor:expr) => {{
        use danube_token::{Token, TokenKind};

        match $cursor.peek() {
            Some(Token {
                span: _,
                kind: TokenKind::Symbol(symbol),
            }) => Some(symbol),
            _ => None,
        }
    }};
    ($cursor:expr => $ident:ident) => {{
        use danube_token::{Symbol, Token, TokenKind};

        match $cursor.peek() {
            Some(Token {
                span: _,
                kind: TokenKind::Symbol(Symbol::$ident),
            }) => {
                $cursor.next();
                true
            }
            _ => false,
        }
    }};
}

#[macro_export]
macro_rules! literal {
    ($cursor:expr) => {{
        use danube_token::{Token, TokenKind};

        match $cursor.peek() {
            Some(Token {
                span: _,
                kind: TokenKind::Literal(literal),
            }) => Some(literal),
            _ => None,
        }
    }};
    ($cursor:expr => $ident:ident) => {{
        use danube_token::{Literal, Token, TokenKind};

        match $cursor.peek() {
            Some(Token {
                span: _,
                kind: TokenKind::Literal($ident),
            }) => {
                $cursor.next();
                true
            }
            _ => false,
        }
    }};
}
