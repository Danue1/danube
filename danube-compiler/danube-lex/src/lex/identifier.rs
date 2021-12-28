use crate::{Cursor, Error, Lex};
use danube_span::Span;
use danube_token::{Token, TokenKind};

impl<'lex> Lex<'lex> {
    pub fn lex_identifier(&mut self) -> Result<Token, Error> {
        let span = lex_identifier_postfix(&mut self.cursor);
        let symbol = self.interner.intern(self.cursor.slice(&span));

        Ok(Token::new(span, TokenKind::Identifier(symbol)))
    }

    pub fn lex_identifier_with_underscore(&mut self) -> Result<Token, Error> {
        let span = lex_identifier_postfix(&mut self.cursor);
        let span = Span::new(span.start - 1, span.end);
        let symbol = self.interner.intern(self.cursor.slice(&span));

        Ok(Token::new(span, TokenKind::Identifier(symbol)))
    }
}

fn lex_identifier_postfix(cursor: &mut Cursor) -> Span {
    let start = cursor.cursor();
    while let Some('a'..='z' | 'A'..='Z' | '0'..='9' | '_') = cursor.peek() {
        cursor.next();
    }
    let end = cursor.cursor();

    Span::new(start, end)
}
