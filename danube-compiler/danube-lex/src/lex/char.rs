use crate::{Error, Lex};
use danube_span::Span;
use danube_token::{LiteralKind, Token, TokenKind};

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
                    Some('\'') => {
                        let span = Span::new(start - 1, start + 3);
                        let symbol = self.intern(c.to_string().as_ref());
                        let kind = TokenKind::Literal(symbol, LiteralKind::Char);

                        Ok(Token::new(span, kind))
                    }
                    _ => Err(Error::Invalid(start + 2)),
                }
            }
            Some(c) => {
                let span = Span::new(start - 1, start + 2);
                let symbol = self.intern(c.to_string().as_ref());
                let kind = TokenKind::Literal(symbol, LiteralKind::Char);

                Ok(Token::new(span, kind))
            }
            _ => Err(Error::Invalid(start)),
        }
    }
}
