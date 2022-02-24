use crate::{Error, Lex};
use danube_span::{Location, Span};
use danube_token::{LiteralKind, Symbol, Token, TokenKind};

impl<'lex> Lex<'lex> {
    pub fn lex_char(&mut self) -> Result<Token, Error> {
        let start = self.cursor.location();

        match self.cursor.next() {
            Some('\\') => {
                let c = match self.cursor.next() {
                    Some('r') => '\r',
                    Some('n') => '\n',
                    Some('t') => '\t',
                    Some(c) => c,
                    None => {
                        let location = self.cursor.location();

                        return Err(Error::Invalid(Location {
                            offset: location.offset - 1,
                            line: location.line,
                            column: location.column - 1,
                        }));
                    }
                };
                match self.cursor.next() {
                    Some('\'') => {
                        let span = Span::new(
                            Location {
                                offset: start.offset - 1,
                                line: start.line,
                                column: start.column - 1,
                            },
                            Location {
                                offset: start.offset + 3,
                                line: start.line,
                                column: start.column + 3,
                            },
                        );
                        let symbol = Symbol::intern(c.to_string().as_ref());
                        let kind = TokenKind::Literal(symbol, LiteralKind::Char);

                        Ok(Token::new(span, kind))
                    }
                    _ => Err(Error::Invalid(Location {
                        offset: start.offset + 2,
                        line: start.line,
                        column: start.column + 2,
                    })),
                }
            }
            Some(c) => {
                let span = Span::new(
                    Location {
                        offset: start.offset - 1,
                        line: start.line,
                        column: start.column - 1,
                    },
                    Location {
                        offset: start.offset + 2,
                        line: start.line,
                        column: start.column + 2,
                    },
                );
                let symbol = Symbol::intern(c.to_string().as_ref());
                let kind = TokenKind::Literal(symbol, LiteralKind::Char);

                Ok(Token::new(span, kind))
            }
            _ => Err(Error::Invalid(start)),
        }
    }
}
