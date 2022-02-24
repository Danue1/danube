use danube_span::{Location, Span};
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone)]
pub struct Cursor<'lex> {
    iter: Peekable<Chars<'lex>>,
    source: &'lex str,
    offset: usize,
    line: usize,
    column: usize,
}

impl<'lex> Cursor<'lex> {
    #[inline]
    pub fn new(source: &'lex str) -> Self {
        Cursor {
            iter: source.chars().peekable(),
            source,
            offset: 0,
            line: 1,
            column: 1,
        }
    }

    #[inline(always)]
    pub fn location(&self) -> Location {
        Location {
            offset: self.offset,
            line: self.line,
            column: self.column,
        }
    }

    #[inline]
    pub fn peek(&mut self) -> Option<&char> {
        self.iter.peek()
    }

    #[inline]
    pub fn slice(&self, span: &Span) -> &'lex str {
        &self.source[span.start.offset..span.end.offset]
    }

    #[inline]
    pub fn advance_line(&mut self) {
        self.offset += 1;
        self.line += 1;
        self.column = 1;
    }
}

impl<'lex> std::iter::Iterator for Cursor<'lex> {
    type Item = char;

    #[inline]
    fn next(&mut self) -> Option<char> {
        self.offset += 1;
        self.column += 1;
        self.iter.next()
    }
}
