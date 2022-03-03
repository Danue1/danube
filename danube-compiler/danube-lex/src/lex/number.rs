use crate::{Cursor, Lex};
use danube_diagnostics::{Message, MessageBuilder};
use danube_span::Span;
use danube_token::{LiteralKind, Symbol, Token, TokenKind};

impl<'lex> Lex<'lex> {
    pub fn lex_number(&mut self) -> Result<Token, Message> {
        let integer_span = lex_numeric(&mut self.cursor);

        match self.cursor.peek() {
            Some('.') => {
                self.cursor.next();
                match self.cursor.peek() {
                    Some('0'..='9') => {
                        let float_span = integer_span.with_end(lex_numeric(&mut self.cursor).end);
                        let string = self.cursor.slice(&float_span);
                        let symbol = Symbol::intern(string);
                        let kind = TokenKind::Literal(symbol, LiteralKind::Float);

                        Ok(Token::new(float_span, kind))
                    }
                    _ => Err(
                        MessageBuilder::error("Expected a digit after the decimal point").build(),
                    ),
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

    pub fn lex_number_without_integer(&mut self) -> Result<Token, Message> {
        let float_span = {
            let mut float_span = lex_numeric(&mut self.cursor);
            float_span.start.decrement();
            float_span
        };
        let string = self.cursor.slice(&float_span);
        let symbol = Symbol::intern(string);
        let kind = TokenKind::Literal(symbol, LiteralKind::Float);

        Ok(Token::new(float_span, kind))
    }
}

fn lex_numeric(cursor: &mut Cursor) -> Span {
    let start = cursor.location();
    while let Some('0'..='9') = cursor.peek() {
        cursor.next();
    }
    let end = cursor.location();

    Span::new(start, end)
}
