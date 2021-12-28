use crate::{Error, Lex};
use danube_span::Span;
use danube_token::{LiteralKind, Token, TokenKind};

impl<'lex> Lex<'lex> {
    pub fn lex_string(&mut self) -> Result<Token, Error> {
        let start = self.cursor.cursor();
        let mut string = String::new();

        loop {
            match self.cursor.next() {
                Some('"') => {
                    let span = Span::new(start - 1, self.cursor.cursor());
                    let symbol = self.intern(string.as_ref());
                    let kind = TokenKind::Literal(symbol, LiteralKind::String);

                    return Ok(Token::new(span, kind));
                }
                Some('\\') => {
                    let c = match self.cursor.next() {
                        Some('r') => '\r',
                        Some('n') => '\n',
                        Some('t') => '\t',
                        Some(c) => c,
                        None => return Err(Error::Invalid(self.cursor.cursor() - 1)),
                    };
                    string.push(c);
                }
                Some(c) => {
                    string.push(c);
                }
                None => return Err(Error::Invalid(self.cursor.cursor())),
            }
        }
    }
}
