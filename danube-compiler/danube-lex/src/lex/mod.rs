mod char;
mod identifier;
mod number;
mod string;

use crate::{Cursor, Error};
use danube_span::Span;
use danube_token::{Symbol, SymbolContainer, SymbolInterner, Token, TokenKind};

pub struct Lex<'lex> {
    interner: SymbolInterner,
    cursor: Cursor<'lex>,
}

impl<'lex> Lex<'lex> {
    pub fn symbols(self) -> SymbolContainer {
        self.interner.into()
    }
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
                Some('a'..='z' | 'A'..='Z') => return Some(self.lex_identifier()),
                Some(_) => return Some(self.lex()),
                None => return None,
            }
        }
    }
}

macro_rules! symbol {
    ($cursor:expr => $kind:ident) => {{
        let symbol = TokenKind::$kind;
        let end = $cursor.cursor();
        let count = match symbol.count() {
            Some(count) => count,
            None => return Err(Error::UnknownSymbol),
        };
        let span = Span::new(end - count, end);

        Ok(Token::new(span, symbol))
    }};
}

impl<'lex> Lex<'lex> {
    pub fn new(source: &'lex str) -> Self {
        Lex {
            interner: Default::default(),
            cursor: Cursor::new(source),
        }
    }

    #[inline(always)]
    fn intern(&mut self, string: &str) -> Symbol {
        self.interner.intern(string)
    }

    #[inline(always)]
    fn lex(&mut self) -> Result<Token, Error> {
        match self.cursor.next().unwrap() {
            '(' => symbol!(self.cursor => LeftParens),
            ')' => symbol!(self.cursor => RightParens),
            '[' => symbol!(&self.cursor => LeftBracket),
            ']' => symbol!(&self.cursor => RightBracket),
            '{' => symbol!(&self.cursor => LeftBrace),
            '}' => symbol!(&self.cursor => RightBrace),
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
                    let start = self.cursor.cursor() - 2;
                    let is_document = matches!(self.cursor.peek(), Some('/'));
                    if is_document {
                        self.cursor.next();
                    }
                    while !matches!(self.cursor.peek(), Some('\r' | '\n') | None) {
                        self.cursor.next();
                    }
                    let end = self.cursor.cursor();
                    let span = Span::new(start, end);
                    let symbol = self.intern(self.cursor.slice(&span));

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
            '_' => match self.cursor.peek() {
                Some('a'..='z' | 'A'..='Z' | '0'..='9') => self.lex_identifier_with_underscore(),
                _ => symbol!(self.cursor => Underscore),
            },
            '"' => self.lex_string(),
            '\'' => self.lex_char(),
            _ => Err(Error::Invalid(self.cursor.cursor())),
        }
    }
}

#[test]
fn skip_whitespace() {
    assert_eq!(Lex::new(" ").next(), None);
    assert_eq!(Lex::new("\r").next(), None);
    assert_eq!(Lex::new("\n").next(), None);
    assert_eq!(Lex::new("\t").next(), None);
    assert_eq!(Lex::new(" \r\n\t").next(), None);
}

#[test]
fn symbols() {
    macro_rules! assert_symbols {
        ($($symbol:expr => $expect:ident,)+) => {
            $(
                let kind = TokenKind::$expect;
                if let Some(count) = kind.count() {
                    assert_eq!(
                        Lex::new($symbol).next(),
                        Some(Ok(Token::new(Span::new(0, count), kind)))
                    );
                } else {
                    // assert with error message for preventing human-mistakes.
                    std::todo!();
                }
            )+
        };
    }

    assert_symbols! {
        "(" => LeftParens,
        ")" => RightParens,
        "[" => LeftBracket,
        "]" => RightBracket,
        "{" => LeftBrace,
        "}" => RightBrace,
        "<" => LeftChevron,
        ">" => RightChevron,
        "#" => Hash,
        "." => Dot,
        "," => Comma,
        ":" => Colon,
        ";" => Semicolon,
        "=" => Eq,
        "+" => Plus,
        "-" => Hyphen,
        "*" => Asterisk,
        "/" => Slash,
        "%" => Percent,
        "!" => Exclamation,
        "?" => Question,
        "&" => Ampersand,
        "|" => Pipeline,
        "~" => Tilde,
        "^" => Caret,
        "_" => Underscore,

        "->" => HyphenRightChevron,
        "=>" => EqRightChevron,
        ".." => DotDot,
        "::" => ColonColon,
        "==" => EqEq,
        "!=" => ExclamationEq,
        "+=" => PlusEq,
        "-=" => HyphenEq,
        "*=" => AsteriskEq,
        "/=" => SlashEq,
        "%=" => PercentEq,
        "**" => AsteriskAsterisk,
        "&&" => AmpersandAmpersand,
        "||" => PipelinePipeline,
        "&=" => AmpersandEq,
        "|=" => PipelineEq,
        "~=" => TildeEq,
        "^=" => CaretEq,
        "<<" => LeftChevronLeftChevron,
        ">>" => RightChevronRightChevron,
        "<=" => LeftChevronEq,
        ">=" => RightChevronEq,

        "..=" => DotDotEq,
        "**=" => AsteriskAsteriskEq,
        "&&=" => AmpersandAmpersandEq,
        "||=" => PipelinePipelineEq,
        "<<=" => LeftChevronLeftChevronEq,
        ">>=" => RightChevronRightChevronEq,
    };
}

#[test]
fn integers() {
    use danube_token::{LiteralKind, SymbolInterner};

    macro_rules! assert_integers {
        ($($integer:expr),+) => {
            $(
                let mut interner = SymbolInterner::default();

                assert_eq!(
                    Lex::new($integer).next(),
                    Some(Ok(Token::new(
                        Span::new(0, $integer.len()),
                        TokenKind::Literal(interner.intern($integer), LiteralKind::Integer),
                    )))
                );
            )+
        };
    }

    assert_integers!["0", "1", "10", "11"];
}

#[test]
fn floatings() {
    macro_rules! assert_floatings {
        ($($floating:expr),+) => {
            use danube_token::{LiteralKind, SymbolInterner};

            $(
                let mut interner = SymbolInterner::default();

                assert_eq!(
                    Lex::new($floating).next(),
                    Some(Ok(Token::new(
                        Span::new(0, $floating.len()),
                        TokenKind::Literal(interner.intern($floating), LiteralKind::Float),
                    )))
                );
            )+
        };
    }

    assert_floatings![".0", ".1", "0.0", "0.1", "1.23"];
}

#[test]
fn chars() {
    macro_rules! assert_chars {
        ($($char:expr),+) => {
            use danube_token::{LiteralKind, SymbolInterner};

            $(
                let mut interner = SymbolInterner::default();

                assert_eq!(
                    Lex::new($char).next(),
                    Some(Ok(Token::new(
                        Span::new(0, $char.len()),
                        TokenKind::Literal(interner.intern($char), LiteralKind::Char),
                    )))
                );
            )+
        };
    }

    assert_chars![r#"'a'"#, r#"'\r'"#, r#"'\n'"#, r#"'\t'"#];
}

#[test]
fn strings() {
    macro_rules! assert_strings {
        ($($string:expr),+) => {
            use danube_token::{LiteralKind, SymbolInterner};

            $(
                let mut interner = SymbolInterner::default();

                assert_eq!(
                    Lex::new($string).next(),
                    Some(Ok(Token::new(
                        Span::new(0, $string.len()),
                        TokenKind::Literal(interner.intern($string), LiteralKind::String),
                    )))
                );
            )+
        };
    }

    assert_strings![r#""Hello, World!""#, r#""\r\t\n""#];
}

#[test]
fn identifiers() {
    macro_rules! tokens {
        ($($identifier:expr),+ $(,)?) => {{
            let mut interner = SymbolInterner::default();
            let mut vec = Vec::new();
            let mut position = 0;
            $(
                let span = Span::new(position, position + $identifier.len());
                #[allow(unused_assignments)]
                {
                    position = span.end + 1;
                }

                let kind = TokenKind::Identifier(interner.intern($identifier));
                vec.push(Token::new(span, kind));

            )+
            vec
        }};
    }

    let source = "_a _1 _a1 _1a a a_ a1 ab ab1";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        tokens,
        tokens!["_a", "_1", "_a1", "_1a", "a", "a_", "a1", "ab", "ab1"]
    );
}

#[test]
fn keywords() {
    macro_rules! assert_keywords {
        ($($keyword:expr),+) => {
            $(
                let mut interner = SymbolInterner::default();

                assert_eq!(
                    Lex::new($keyword).next(),
                    Some(Ok(Token::new(
                        Span::new(0, $keyword.len()),
                        TokenKind::Identifier(interner.intern($keyword)),
                    )))
                );
            )+
        };
    }

    assert_keywords![
        "if", "else", "for", "while", "loop", "in", "break", "continue", "match", "return",
        "yield", "where", "const", "let", "mut", "enum", "struct", "fn", "Self", "self", "use",
        "super", "pub", "as", "package", "type", "trait", "impl"
    ];
}

#[test]
fn singleline_comment() {
    macro_rules! assert_comments {
        ($($comment:expr),+) => {
            $(
                let mut interner = SymbolInterner::default();

                assert_eq!(Lex::new($comment).next(), Some(Ok(Token::new(
                    Span::new(0, $comment.len()),
                    TokenKind::Comment(interner.intern($comment))
                ))));
            )+
        };
    }

    assert_comments!["//", "///", "//hello", "///hello"];
}
