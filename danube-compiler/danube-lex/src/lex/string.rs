use crate::{Error, Lex};
use danube_token::{Literal, Span, Token, TokenKind};

impl<'lex> Lex<'lex> {
    pub fn lex_string(&mut self) -> Result<Token, Error> {
        let start = self.cursor.cursor();
        let mut string = String::new();

        loop {
            match self.cursor.next() {
                Some('"') => {
                    return Ok(Token::new(
                        Span::new(start - 1, self.cursor.cursor()),
                        TokenKind::Literal(Literal::String(string)),
                    ))
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
                Some(c) => string.push(c),
                None => return Err(Error::Invalid(self.cursor.cursor())),
            }
        }
    }
}
