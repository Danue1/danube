use crate::Error;
use danube_lex::LexIter;
use danube_token::{Keyword, Symbol, Token, TokenKind};

pub(crate) struct Cursor<'parse> {
    iter: LexIter<'parse>,
}

impl<'parse> Cursor<'parse> {
    #[inline(always)]
    pub(crate) fn new(source: &'parse str) -> Self {
        Cursor {
            iter: LexIter::new(source),
        }
    }

    #[inline(always)]
    pub(crate) fn error(&mut self) -> Error {
        match self.peek() {
            Ok(Some(token)) => Error::Illegal(token),
            Ok(None) => Error::Invalid,
            Err(error) => error,
        }
    }

    #[inline(always)]
    pub(crate) fn expect_keyword(&mut self, keyword: Keyword) -> bool {
        match self.peek() {
            Ok(Some(Token {
                kind: TokenKind::Keyword(current),
                ..
            })) => current == keyword,
            _ => false,
        }
    }

    #[inline(always)]
    pub(crate) fn expect_symbol(&mut self, symbol: Symbol) -> bool {
        match self.peek() {
            Ok(Some(Token {
                kind: TokenKind::Symbol(current),
                ..
            })) => current == symbol,
            _ => false,
        }
    }

    #[inline(always)]
    pub(crate) fn consume_keyword(&mut self, keyword: Keyword) -> Result<(), Error> {
        match self.peek() {
            Ok(Some(Token {
                kind: TokenKind::Keyword(current),
                ..
            })) if current == keyword => {
                self.next();
                Ok(())
            }
            _ => Err(self.error()),
        }
    }

    #[inline(always)]
    pub(crate) fn consume_symbol(&mut self, symbol: Symbol) -> Result<(), Error> {
        match self.peek() {
            Ok(Some(Token {
                kind: TokenKind::Symbol(current),
                ..
            })) if current == symbol => {
                self.next();
                Ok(())
            }
            _ => Err(self.error()),
        }
    }

    #[inline(always)]
    pub(crate) fn consume_ident(&mut self) -> Result<String, Error> {
        match self.peek() {
            Ok(Some(Token {
                kind: TokenKind::Identifier(current),
                ..
            })) => {
                self.next();
                Ok(current)
            }
            _ => Err(self.error()),
        }
    }

    #[inline(always)]
    pub(crate) fn next(&mut self) {
        self.iter.next();
    }

    #[inline(always)]
    pub(crate) fn peek(&mut self) -> Result<Option<Token>, Error> {
        match self.iter.clone().next() {
            Some(Ok(token)) => Ok(Some(token)),
            Some(Err(error)) => Err(Error::Lex(error)),
            None => Ok(None),
        }
    }

    #[inline(always)]
    pub(crate) fn clone(&mut self) -> Self {
        Cursor {
            iter: self.iter.clone(),
        }
    }
}
