use danube_token::{Token, EOF};
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
    pub fn peek(&mut self) -> &Token {
        match self.iter.peek() {
            Some(token) => token,
            None => &EOF,
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
        use danube_token::TokenKind;

        if let TokenKind::Identifier(identifier) = $cursor.peek().kind {
            Some(identifier)
        } else {
            None
        }
    }};

    ($cursor:expr => $keyword:ident) => {{
        use danube_token::TokenKind;

        if $cursor.peek().kind == TokenKind::Identifier(danube_token::keywords::$keyword) {
            $cursor.next();
            true
        } else {
            false
        }
    }};
}

#[macro_export]
macro_rules! symbol {
    ($cursor:expr) => {{
        use danube_token::TokenKind;

        let kind = &$cursor.peek().kind;
        if matches!(
            kind,
            TokenKind::Identifier(_) | TokenKind::Literal(_, _) | TokenKind::Comment(_)
        ) {
            None
        } else {
            Some(kind)
        }
    }};

    ($cursor:expr => $ident:ident) => {{
        use danube_token::TokenKind;

        if $cursor.peek().kind == TokenKind::$ident {
            $cursor.next();
            true
        } else {
            false
        }
    }};
}

#[macro_export]
macro_rules! literal {
    ($cursor:expr) => {{
        use danube_token::{Token, TokenKind};

        if let TokenKind::Literal(literal) = $cursor.peek().kind {
            Some(literal)
        } else {
            None
        }
    }};

    ($cursor:expr => $ident:ident) => {{
        use danube_token::{Literal, Token, TokenKind};

        if $cursor.peek().kind == TokenKind::Literal($ident) {
            $cursor.next();
            true
        } else {
            false
        }
    }};
}
