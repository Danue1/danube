use crate::{Error, Lex};
use danube_token::{Literal, Span, Token, TokenKind};

impl<'lex> Lex<'lex> {
    pub fn lex_char(&mut self) -> Result<Token, Error> {
        let start = self.cursor.cursor();

        match self.cursor.next() {
            Some('\\') => {
                let c = match self.cursor.next() {
                    Some('r') => '\r',
                    Some('n') => '\n',
                    Some('t') => '\t',
                    Some(c) => c,
                    None => return Err(Error::Invalid(self.cursor.cursor() - 1)),
                };
                match self.cursor.next() {
                    Some('\'') => Ok(Token::new(
                        Span::new(start - 1, start + 3),
                        TokenKind::Literal(Literal::Char(c)),
                    )),
                    _ => Err(Error::Invalid(start + 2)),
                }
            }
            Some(c) => match self.cursor.next() {
                Some('\'') => Ok(Token::new(
                    Span::new(start - 1, start + 2),
                    TokenKind::Literal(Literal::Char(c)),
                )),
                _ => Err(Error::Invalid(start + 1)),
            },
            _ => Err(Error::Invalid(start)),
        }
    }
}
