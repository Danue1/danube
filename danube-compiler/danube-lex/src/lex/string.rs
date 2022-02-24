use crate::{Error, Lex};
use danube_span::{Location, Span};
use danube_token::{LiteralKind, Symbol, Token, TokenKind};

impl<'lex> Lex<'lex> {
    pub fn lex_string(&mut self) -> Result<Token, Error> {
        let start = self.cursor.location();
        let mut string = String::new();

        loop {
            match self.cursor.next() {
                Some('"') => {
                    let span = {
                        let start = Location {
                            offset: start.offset - 1,
                            line: start.line,
                            column: start.column - 1,
                        };
                        Span::new(start, self.cursor.location())
                    };
                    let symbol = Symbol::intern(string.as_ref());
                    let kind = TokenKind::Literal(symbol, LiteralKind::String);

                    return Ok(Token::new(span, kind));
                }
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
                    string.push(c);
                }
                Some(c) => {
                    string.push(c);
                }
                None => return Err(Error::Invalid(self.cursor.location())),
            }
        }
    }
}
