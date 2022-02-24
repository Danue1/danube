use crate::{Cursor, Error, Lex};
use danube_span::Span;
use danube_token::{Symbol, Token, TokenKind};

impl<'lex> Lex<'lex> {
    pub fn lex_identifier(&mut self) -> Result<Token, Error> {
        let span = lex_identifier_postfix(&mut self.cursor);
        let symbol = Symbol::intern(self.cursor.slice(&span));

        Ok(Token::new(span, TokenKind::Identifier(symbol)))
    }
}

fn lex_identifier_postfix(cursor: &mut Cursor) -> Span {
    let start = cursor.location();
    while let Some('a'..='z' | 'A'..='Z' | '0'..='9' | '_') = cursor.peek() {
        cursor.next();
    }
    let end = cursor.location();

    Span::new(start, end)
}
