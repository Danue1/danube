use crate::{Cursor, Error, Lex};
use danube_token::{Literal, Span, Token, TokenKind};

impl<'lex> Lex<'lex> {
    pub fn lex_number(&mut self) -> Result<Token, Error> {
        let integer_span = lex_numeric(&mut self.cursor);

        match self.cursor.peek() {
            Some('.') => {
                self.cursor.next();
                match self.cursor.peek() {
                    Some('0'..='9') => {
                        let float_span = Span::concat(integer_span, lex_numeric(&mut self.cursor));
                        match self.cursor.slice(&float_span).parse() {
                            Ok(float) => Ok(Token::new(
                                float_span,
                                TokenKind::Literal(Literal::Float(float)),
                            )),
                            Err(_) => Err(Error::MalformedFloating(float_span)),
                        }
                    }
                    _ => Err(Error::Invalid(self.cursor.cursor())),
                }
            }
            _ => match self.cursor.slice(&integer_span).parse() {
                Ok(integer) => Ok(Token::new(
                    integer_span,
                    TokenKind::Literal(Literal::Integer(integer)),
                )),
                Err(_) => Err(Error::MalformedInteger(integer_span)),
            },
        }
    }

    pub fn lex_number_without_integer(&mut self) -> Result<Token, Error> {
        let float_span = lex_numeric(&mut self.cursor);
        let float_span = Span::new(float_span.start - 1, float_span.end);

        match self.cursor.slice(&float_span).parse() {
            Ok(float) => Ok(Token::new(
                float_span,
                TokenKind::Literal(Literal::Float(float)),
            )),
            Err(_) => Err(Error::MalformedFloating(float_span)),
        }
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
