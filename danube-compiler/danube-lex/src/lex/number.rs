use crate::{Cursor, Error, Lex};
use danube_span::Span;
use danube_token::{LiteralKind, Symbol, Token, TokenKind};

impl<'lex> Lex<'lex> {
    pub fn lex_number(&mut self) -> Result<Token, Error> {
        let integer_span = lex_numeric(&mut self.cursor);

        match self.cursor.peek() {
            Some('.') => {
                self.cursor.next();
                match self.cursor.peek() {
                    Some('0'..='9') => {
                        let float_span = integer_span.concat(lex_numeric(&mut self.cursor));
                        let string = self.cursor.slice(&float_span);
                        let symbol = Symbol::intern(string);
                        let kind = TokenKind::Literal(symbol, LiteralKind::Float);

                        Ok(Token::new(float_span, kind))
                    }
                    _ => Err(Error::Invalid(self.cursor.cursor())),
                }
            }
            _ => {
                let string = self.cursor.slice(&integer_span);
                let symbol = Symbol::intern(string);
                let kind = TokenKind::Literal(symbol, LiteralKind::Integer);

                Ok(Token::new(integer_span, kind))
            }
        }
    }

    pub fn lex_number_without_integer(&mut self) -> Result<Token, Error> {
        let float_span = lex_numeric(&mut self.cursor);
        let float_span = Span::new(float_span.start - 1, float_span.end);
        let string = self.cursor.slice(&float_span);
        let symbol = Symbol::intern(string);
        let kind = TokenKind::Literal(symbol, LiteralKind::Float);

        Ok(Token::new(float_span, kind))
    }
}

fn lex_numeric(cursor: &mut Cursor) -> Span {
    let start = cursor.cursor();
    while let Some('0'..='9') = cursor.peek() {
        cursor.next();
    }
    let end = cursor.cursor();

    Span::new(start, end)
}
