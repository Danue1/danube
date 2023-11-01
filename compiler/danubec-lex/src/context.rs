use danubec_token::{Token, TokenKind, EOF};
use std::ops::Range;

pub struct Context<'lex> {
    index: usize,
    source: &'lex str,
    tokens: Vec<Token<'lex>>,
}

impl<'lex> Context<'lex> {
    pub fn new(source: &'lex str) -> Self {
        Self {
            index: 0,
            source,
            tokens: vec![],
        }
    }

    pub fn bump(&mut self, kind: TokenKind) {
        let len = self.peek().map(|c| c.len_utf8()).unwrap_or(0);
        self.tokens.push(Token {
            kind,
            source: &self.source[self.index..self.index + len],
        });
        self.index += len;
    }

    pub fn bump_with(&mut self, kind: TokenKind, range: Range<usize>) {
        self.index = range.end;
        self.tokens.push(Token {
            kind,
            source: &self.source[range],
        });
    }

    #[inline]
    pub fn peek(&mut self) -> Option<char> {
        self.source[self.index..].chars().next()
    }

    #[inline]
    pub fn advanced(&mut self) {
        self.index += 1;
    }

    #[inline]
    pub fn build(mut self) -> Vec<Token<'lex>> {
        self.tokens.push(EOF);
        self.tokens
    }

    #[inline]
    pub const fn index(&self) -> usize {
        self.index
    }
}
