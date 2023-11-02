use danubec_syntax_kind::SyntaxKind;
use std::ops::Range;

pub struct Context<'lex> {
    index: usize,
    source: &'lex str,
    tokens: Vec<(SyntaxKind, String)>,
}

impl<'lex> Context<'lex> {
    pub fn new(source: &'lex str) -> Self {
        Self {
            index: 0,
            source,
            tokens: vec![],
        }
    }

    pub fn bump(&mut self, kind: SyntaxKind) {
        let len = self.peek().map(|c| c.len_utf8()).unwrap_or(0);
        self.bump_with(kind, self.index..self.index + len);
    }

    pub fn bump_with(&mut self, kind: SyntaxKind, range: Range<usize>) {
        self.index = range.end;
        self.tokens.push((kind, self.source[range].to_owned()));
    }

    pub fn slice(&self, range: Range<usize>) -> &'lex str {
        &self.source[range]
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
    pub fn build(self) -> Vec<(SyntaxKind, String)> {
        self.tokens
    }

    pub const fn index(&self) -> usize {
        self.index
    }
}
