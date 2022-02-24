mod char;
mod identifier;
mod number;
mod string;

#[cfg(test)]
mod tests;

use crate::{Cursor, Error};
use danube_span::{Location, Span};
use danube_token::{Symbol, Token, TokenKind};

pub struct Lex<'lex> {
    cursor: Cursor<'lex>,
}

impl<'lex> std::iter::Iterator for Lex<'lex> {
    type Item = Result<Token, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.cursor.peek() {
                Some(' ' | '\r' | '\n' | '\t') => {
                    self.cursor.next();
                }
                Some('0'..='9') => return Some(self.lex_number()),
                Some('a'..='z' | 'A'..='Z' | '_') => return Some(self.lex_identifier()),
                Some(_) => return Some(self.lex()),
                None => return None,
            }
        }
    }
}

macro_rules! symbol {
    ($cursor:expr => $kind:ident) => {{
        let symbol = TokenKind::$kind;
        let end = $cursor.location();
        let count = match symbol.count() {
            Some(count) => count,
            None => return Err(Error::UnknownSymbol),
        };
        let start = Location {
            offset: end.offset - count,
            line: end.line,
            column: end.column - count,
        };
        let span = Span::new(start, end);

        Ok(Token::new(span, symbol))
    }};
}

impl<'lex> Lex<'lex> {
    pub fn new(source: &'lex str) -> Self {
        Lex {
            cursor: Cursor::new(source),
        }
    }

    #[inline(always)]
    fn lex(&mut self) -> Result<Token, Error> {
        match self.cursor.next().unwrap() {
            '(' => symbol!(self.cursor => LeftParens),
            ')' => symbol!(self.cursor => RightParens),
            '[' => symbol!(self.cursor => LeftBracket),
            ']' => symbol!(self.cursor => RightBracket),
            '{' => symbol!(self.cursor => LeftBrace),
            '}' => symbol!(self.cursor => RightBrace),
            '<' => match self.cursor.peek() {
                Some('<') => {
                    self.cursor.next();
                    match self.cursor.peek() {
                        Some('=') => {
                            self.cursor.next();
                            symbol!(self.cursor => LeftChevronLeftChevronEq)
                        }
                        _ => symbol!(self.cursor => LeftChevronLeftChevron),
                    }
                }
                Some('=') => {
                    self.cursor.next();
                    symbol!(self.cursor => LeftChevronEq)
                }
                _ => symbol!(self.cursor => LeftChevron),
            },
            '>' => match self.cursor.peek() {
                Some('>') => {
                    self.cursor.next();
                    match self.cursor.peek() {
                        Some('=') => {
                            self.cursor.next();
                            symbol!(self.cursor => RightChevronRightChevronEq)
                        }
                        _ => symbol!(self.cursor => RightChevronRightChevron),
                    }
                }
                Some('=') => {
                    self.cursor.next();
                    symbol!(self.cursor => RightChevronEq)
                }
                _ => symbol!(self.cursor => RightChevron),
            },
            '#' => symbol!(self.cursor => Hash),
            '.' => match self.cursor.peek() {
                Some('.') => {
                    self.cursor.next();
                    match self.cursor.peek() {
                        Some('=') => {
                            self.cursor.next();
                            symbol!(self.cursor => DotDotEq)
                        }
                        _ => symbol!(self.cursor => DotDot),
                    }
                }
                Some('0'..='9') => self.lex_number_without_integer(),
                _ => symbol!(self.cursor => Dot),
            },
            ',' => symbol!(self.cursor => Comma),
            ':' => match self.cursor.peek() {
                Some(':') => {
                    self.cursor.next();
                    symbol!(self.cursor => ColonColon)
                }
                _ => symbol!(self.cursor => Colon),
            },
            ';' => symbol!(self.cursor => Semicolon),
            '=' => match self.cursor.peek() {
                Some('>') => {
                    self.cursor.next();
                    symbol!(self.cursor => EqRightChevron)
                }
                Some('=') => {
                    self.cursor.next();
                    symbol!(self.cursor => EqEq)
                }
                _ => symbol!(self.cursor => Eq),
            },
            '+' => match self.cursor.peek() {
                Some('=') => {
                    self.cursor.next();
                    symbol!(self.cursor => PlusEq)
                }
                _ => symbol!(self.cursor => Plus),
            },
            '-' => match self.cursor.peek() {
                Some('=') => {
                    self.cursor.next();
                    symbol!(self.cursor => HyphenEq)
                }
                Some('>') => {
                    self.cursor.next();
                    symbol!(self.cursor => HyphenRightChevron)
                }
                _ => symbol!(self.cursor => Hyphen),
            },
            '*' => match self.cursor.peek() {
                Some('*') => {
                    self.cursor.next();
                    match self.cursor.peek() {
                        Some('=') => {
                            self.cursor.next();
                            symbol!(self.cursor => AsteriskAsteriskEq)
                        }
                        _ => symbol!(self.cursor => AsteriskAsterisk),
                    }
                }
                Some('=') => {
                    self.cursor.next();
                    symbol!(self.cursor => AsteriskEq)
                }
                _ => symbol!(self.cursor => Asterisk),
            },
            '/' => match self.cursor.peek() {
                Some('/') => {
                    self.cursor.next();
                    let start = {
                        let location = self.cursor.location();

                        Location {
                            offset: location.offset - 2,
                            line: location.line,
                            column: location.column - 2,
                        }
                    };
                    let is_document = matches!(self.cursor.peek(), Some('/'));
                    if is_document {
                        self.cursor.next();
                    }
                    while !matches!(self.cursor.peek(), Some('\r' | '\n') | None) {
                        self.cursor.next();
                    }
                    let end = self.cursor.location();
                    let span = Span::new(start, end);
                    let symbol = Symbol::intern(self.cursor.slice(&span));

                    Ok(Token::new(span, TokenKind::Comment(symbol)))
                }
                Some('*') => {
                    self.cursor.next();

                    std::todo!();
                }
                Some('=') => {
                    self.cursor.next();
                    symbol!(self.cursor => SlashEq)
                }
                _ => symbol!(self.cursor => Slash),
            },
            '%' => match self.cursor.peek() {
                Some('=') => {
                    self.cursor.next();
                    symbol!(self.cursor => PercentEq)
                }
                _ => symbol!(self.cursor => Percent),
            },
            '!' => match self.cursor.peek() {
                Some('=') => {
                    self.cursor.next();
                    symbol!(self.cursor => ExclamationEq)
                }
                _ => symbol!(self.cursor => Exclamation),
            },
            '?' => symbol!(self.cursor => Question),
            '&' => match self.cursor.peek() {
                Some('&') => {
                    self.cursor.next();
                    match self.cursor.peek() {
                        Some('=') => {
                            self.cursor.next();
                            symbol!(self.cursor => AmpersandAmpersandEq)
                        }
                        _ => symbol!(self.cursor => AmpersandAmpersand),
                    }
                }
                Some('=') => {
                    self.cursor.next();
                    symbol!(self.cursor => AmpersandEq)
                }
                _ => symbol!(self.cursor => Ampersand),
            },
            '|' => match self.cursor.peek() {
                Some('|') => {
                    self.cursor.next();
                    match self.cursor.peek() {
                        Some('=') => {
                            self.cursor.next();
                            symbol!(self.cursor => PipelinePipelineEq)
                        }
                        _ => symbol!(self.cursor => PipelinePipeline),
                    }
                }
                Some('=') => {
                    self.cursor.next();
                    symbol!(self.cursor => PipelineEq)
                }
                _ => symbol!(self.cursor => Pipeline),
            },
            '~' => match self.cursor.peek() {
                Some('=') => {
                    self.cursor.next();
                    symbol!(self.cursor => TildeEq)
                }
                _ => symbol!(self.cursor => Tilde),
            },
            '^' => match self.cursor.peek() {
                Some('=') => {
                    self.cursor.next();
                    symbol!(self.cursor => CaretEq)
                }
                _ => symbol!(self.cursor => Caret),
            },
            '"' => self.lex_string(),
            '\'' => self.lex_char(),
            _ => Err(Error::Invalid(self.cursor.location())),
        }
    }
}
