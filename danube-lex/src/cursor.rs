use crate::Error;
use danube_token::Span;
use std::str::Chars;

#[derive(Clone)]
pub(crate) struct Cursor<'lex> {
    source: &'lex str,
    chars: Chars<'lex>,
    position: usize,
}

impl<'lex> Cursor<'lex> {
    pub(crate) fn new(source: &'lex str) -> Self {
        Cursor {
            source,
            chars: source.chars(),
            position: 0,
        }
    }

    #[inline]
    pub(crate) fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }

    #[inline]
    pub(crate) fn next(&mut self) {
        self.chars.next();
        self.position += 1;
    }

    #[inline]
    pub(crate) fn position(&self) -> usize {
        self.position
    }

    #[inline]
    pub(crate) fn slice(&mut self, range: std::ops::Range<usize>) -> &str {
        &self.source[range]
    }

    #[inline]
    pub(crate) fn consume_while<F>(&mut self, predicate: F)
    where
        F: Fn(char) -> bool,
    {
        while let Some(peek) = self.peek() {
            if predicate(peek) {
                self.next();
            } else {
                break;
            }
        }
    }

    #[inline]
    pub(crate) fn span_by<T, F>(&mut self, f: F) -> Result<(Span, T), Error>
    where
        F: FnOnce(&mut Self) -> Result<T, Error>,
    {
        let start = self.position;
        let ret = f(self)?;
        let end = self.position;

        Ok((Span::new(start, end), ret))
    }
}
