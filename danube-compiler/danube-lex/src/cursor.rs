use danube_token::Span;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone)]
pub struct Cursor<'lex> {
    source: &'lex str,
    cursor: usize,
    iter: Peekable<Chars<'lex>>,
}

impl<'lex> Cursor<'lex> {
    #[inline]
    pub fn new(source: &'lex str) -> Self {
        Cursor {
            source,
            cursor: 0,
            iter: source.chars().peekable(),
        }
    }

    #[inline(always)]
    pub fn cursor(&self) -> usize {
        self.cursor
    }

    #[inline]
    pub fn peek(&mut self) -> Option<&char> {
        self.iter.peek()
    }

    #[inline]
    pub fn slice(&self, span: &Span) -> &str {
        &self.source[span.start..span.end]
    }
}

impl<'lex> std::iter::Iterator for Cursor<'lex> {
    type Item = char;

    #[inline]
    fn next(&mut self) -> Option<char> {
        self.cursor += 1;
        self.iter.next()
    }
}
