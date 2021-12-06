mod char;
mod identifier;
mod number;
mod string;

use crate::{Cursor, Error};
use danube_token::{Comment, Span, Symbol, Token, TokenKind};

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
                Some('0'..='9') => {
                    return match self.lex_number() {
                        Ok(token) => Some(Ok(token)),
                        Err(error) => Some(Err(error)),
                    }
                }
                Some('a'..='z' | 'A'..='Z') => {
                    return match self.lex_identifier() {
                        Ok(token) => Some(Ok(token)),
                        Err(error) => Some(Err(error)),
                    }
                }
                Some(_) => {
                    return match self.lex() {
                        Ok(token) => Some(Ok(token)),
                        Err(error) => Some(Err(error)),
                    }
                }
                _ => return None,
            }
        }
    }
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
            '(' => Ok(symbol(&self.cursor, Symbol::LeftParens)),
            ')' => Ok(symbol(&self.cursor, Symbol::RightParens)),
            '[' => Ok(symbol(&self.cursor, Symbol::LeftBracket)),
            ']' => Ok(symbol(&self.cursor, Symbol::RightBracket)),
            '{' => Ok(symbol(&self.cursor, Symbol::LeftBrace)),
            '}' => Ok(symbol(&self.cursor, Symbol::RightBrace)),
            '<' => match self.cursor.peek() {
                Some('<') => {
                    self.cursor.next();
                    match self.cursor.peek() {
                        Some('=') => {
                            self.cursor.next();
                            Ok(symbol(&self.cursor, Symbol::LeftChevronLeftChevronEq))
                        }
                        _ => Ok(symbol(&self.cursor, Symbol::LeftChevronLeftChevron)),
                    }
                }
                Some('=') => {
                    self.cursor.next();
                    Ok(symbol(&self.cursor, Symbol::LeftChevronEq))
                }
                _ => Ok(symbol(&self.cursor, Symbol::LeftChevron)),
            },
            '>' => match self.cursor.peek() {
                Some('>') => {
                    self.cursor.next();
                    match self.cursor.peek() {
                        Some('=') => {
                            self.cursor.next();
                            Ok(symbol(&self.cursor, Symbol::RightChevronRightChevronEq))
                        }
                        _ => Ok(symbol(&self.cursor, Symbol::RightChevronRightChevron)),
                    }
                }
                Some('=') => {
                    self.cursor.next();
                    Ok(symbol(&self.cursor, Symbol::RightChevronEq))
                }
                _ => Ok(symbol(&self.cursor, Symbol::RightChevron)),
            },
            '#' => Ok(symbol(&self.cursor, Symbol::Hash)),
            '.' => match self.cursor.peek() {
                Some('.') => {
                    self.cursor.next();
                    match self.cursor.peek() {
                        Some('=') => {
                            self.cursor.next();
                            Ok(symbol(&self.cursor, Symbol::DotDotEq))
                        }
                        _ => Ok(symbol(&self.cursor, Symbol::DotDot)),
                    }
                }
                Some('0'..='9') => self.lex_number_without_integer(),
                _ => Ok(symbol(&self.cursor, Symbol::Dot)),
            },
            ',' => Ok(symbol(&self.cursor, Symbol::Comma)),
            ':' => match self.cursor.peek() {
                Some(':') => {
                    self.cursor.next();
                    Ok(symbol(&self.cursor, Symbol::ColonColon))
                }
                _ => Ok(symbol(&self.cursor, Symbol::Colon)),
            },
            ';' => Ok(symbol(&self.cursor, Symbol::Semicolon)),
            '=' => match self.cursor.peek() {
                Some('>') => {
                    self.cursor.next();
                    Ok(symbol(&self.cursor, Symbol::EqRightChevron))
                }
                Some('=') => {
                    self.cursor.next();
                    Ok(symbol(&self.cursor, Symbol::EqEq))
                }
                _ => Ok(symbol(&self.cursor, Symbol::Eq)),
            },
            '+' => match self.cursor.peek() {
                Some('=') => {
                    self.cursor.next();
                    Ok(symbol(&self.cursor, Symbol::PlusEq))
                }
                _ => Ok(symbol(&self.cursor, Symbol::Plus)),
            },
            '-' => match self.cursor.peek() {
                Some('=') => {
                    self.cursor.next();
                    Ok(symbol(&self.cursor, Symbol::HyphenEq))
                }
                Some('>') => {
                    self.cursor.next();
                    Ok(symbol(&self.cursor, Symbol::HyphenRightChevron))
                }
                _ => Ok(symbol(&self.cursor, Symbol::Hyphen)),
            },
            '*' => match self.cursor.peek() {
                Some('*') => {
                    self.cursor.next();
                    match self.cursor.peek() {
                        Some('=') => {
                            self.cursor.next();
                            Ok(symbol(&self.cursor, Symbol::AsteriskAsteriskEq))
                        }
                        _ => Ok(symbol(&self.cursor, Symbol::AsteriskAsterisk)),
                    }
                }
                Some('=') => {
                    self.cursor.next();
                    Ok(symbol(&self.cursor, Symbol::AsteriskEq))
                }
                _ => Ok(symbol(&self.cursor, Symbol::Asterisk)),
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

                    Ok(Token::new(
                        Span::new(start, end),
                        TokenKind::Comment(Comment::new_singleline(is_document)),
                    ))
                }
                Some('*') => {
                    self.cursor.next();
                    std::todo!();
                }
                Some('=') => {
                    self.cursor.next();
                    Ok(symbol(&self.cursor, Symbol::SlashEq))
                }
                _ => Ok(symbol(&self.cursor, Symbol::Slash)),
            },
            '%' => match self.cursor.peek() {
                Some('=') => {
                    self.cursor.next();
                    Ok(symbol(&self.cursor, Symbol::PercentEq))
                }
                _ => Ok(symbol(&self.cursor, Symbol::Percent)),
            },
            '!' => match self.cursor.peek() {
                Some('=') => {
                    self.cursor.next();
                    Ok(symbol(&self.cursor, Symbol::ExclamationEq))
                }
                _ => Ok(symbol(&self.cursor, Symbol::Exclamation)),
            },
            '?' => Ok(symbol(&self.cursor, Symbol::Question)),
            '&' => match self.cursor.peek() {
                Some('&') => {
                    self.cursor.next();
                    match self.cursor.peek() {
                        Some('=') => {
                            self.cursor.next();
                            Ok(symbol(&self.cursor, Symbol::AmpersandAmpersandEq))
                        }
                        _ => Ok(symbol(&self.cursor, Symbol::AmpersandAmpersand)),
                    }
                }
                Some('=') => {
                    self.cursor.next();
                    Ok(symbol(&self.cursor, Symbol::AmpersandEq))
                }
                _ => Ok(symbol(&self.cursor, Symbol::Ampersand)),
            },
            '|' => match self.cursor.peek() {
                Some('|') => {
                    self.cursor.next();
                    match self.cursor.peek() {
                        Some('=') => {
                            self.cursor.next();
                            Ok(symbol(&self.cursor, Symbol::PipelinePipelineEq))
                        }
                        _ => Ok(symbol(&self.cursor, Symbol::PipelinePipeline)),
                    }
                }
                Some('=') => {
                    self.cursor.next();
                    Ok(symbol(&self.cursor, Symbol::PipelineEq))
                }
                _ => Ok(symbol(&self.cursor, Symbol::Pipeline)),
            },
            '~' => match self.cursor.peek() {
                Some('=') => {
                    self.cursor.next();
                    Ok(symbol(&self.cursor, Symbol::TildeEq))
                }
                _ => Ok(symbol(&self.cursor, Symbol::Tilde)),
            },
            '^' => match self.cursor.peek() {
                Some('=') => {
                    self.cursor.next();
                    Ok(symbol(&self.cursor, Symbol::CaretEq))
                }
                _ => Ok(symbol(&self.cursor, Symbol::Caret)),
            },
            '_' => match self.cursor.peek() {
                Some('a'..='z' | 'A'..='Z' | '0'..='9') => self.lex_identifier_with_underscore(),
                _ => Ok(symbol(&self.cursor, Symbol::Underscore)),
            },
            '"' => self.lex_string(),
            '\'' => self.lex_char(),
            _ => Err(Error::Invalid(self.cursor.cursor())),
        }
    }
}

fn symbol(cursor: &Cursor, symbol: Symbol) -> Token {
    let end = cursor.cursor();
    let span = Span::new(end - symbol.count(), end);
    let kind = TokenKind::Symbol(symbol);

    Token::new(span, kind)
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
                assert_eq!(
                    Lex::new($symbol).next(),
                    Some(Ok(Token::new(
                        Span::new(0, Symbol::$expect.count()),
                        TokenKind::Symbol(Symbol::$expect)
                    )))
                );
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
    use danube_token::Literal;

    macro_rules! assert_integers {
        ($($integer:expr => $expect:expr,)+) => {
            $(
                assert_eq!(
                    Lex::new($integer).next(),
                    Some(Ok(Token::new(
                        Span::new(0, $integer.len()),
                        TokenKind::Literal(Literal::Integer($expect)),
                    )))
                );
            )+
        };
    }

    assert_integers! {
        "0" => 0,
        "1" => 1,
        "10" => 10,
        "11" => 11,
    };
}

#[test]
fn floatings() {
    use danube_token::Literal;

    macro_rules! assert_floatings {
        ($($floating:expr => $expect:expr,)+) => {
            $(
                assert_eq!(
                    Lex::new($floating).next(),
                    Some(Ok(Token::new(
                        Span::new(0, $floating.len()),
                        TokenKind::Literal(Literal::Float($expect)),
                    )))
                );
            )+
        };
    }

    assert_floatings! {
        ".0" => 0.0,
        ".1" => 0.1,
        "0.0" => 0.0,
        "0.1" => 0.1,
        "1.23" => 1.23,
    };
}

#[test]
fn chars() {
    use danube_token::Literal;

    macro_rules! assert_chars {
        ($($char:expr => $expect:expr,)+) => {
            $(
                assert_eq!(
                    Lex::new($char).next(),
                    Some(Ok(Token::new(
                        Span::new(0, $char.len()),
                        TokenKind::Literal(Literal::Char($expect)),
                    )))
                );
            )+
        };
    }

    assert_chars! {
        r#"'a'"# => 'a',
        r#"'\r'"# => '\r',
        r#"'\n'"# => '\n',
        r#"'\t'"# => '\t',
    };
}

#[test]
fn strings() {
    use danube_token::Literal;

    macro_rules! assert_strings {
        ($($string:expr => $expect:expr,)+) => {
            $(
                assert_eq!(
                    Lex::new($string).next(),
                    Some(Ok(Token::new(
                        Span::new(0, $string.len()),
                        TokenKind::Literal(Literal::String($expect.to_string())),
                    )))
                );
            )+
        };
    }

    assert_strings! {
        r#""Hello, World!""# => "Hello, World!",
        r#""\r\t\n""# => "\r\t\n",
    };
}

#[test]
fn identifiers() {
    use danube_token::Identifier;

    macro_rules! tokens {
        ($($identifier:expr),+ $(,)?) => {{
            let mut vec = Vec::new();
            let mut position = 0;
            $(
                let span = Span::new(position, position + $identifier.len());
                #[allow(unused_assignments)]
                {
                    position = span.end + 1;
                }

                let kind = TokenKind::Identifier(Identifier::new($identifier));
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
    use danube_token::Keyword;

    macro_rules! assert_keywords {
        ($($keyword:expr => $expect:ident,)+) => {
            $(
                assert_eq!(
                    Lex::new($keyword).next(),
                    Some(Ok(Token::new(
                        Span::new(0, $keyword.len()),
                        TokenKind::Keyword(Keyword::$expect),
                    )))
                );
            )+
        };
    }

    assert_keywords! {
        "if" => If,
        "else" => Else,
        "for" => For,
        "while" => While,
        "loop" => Loop,
        "in" => In,
        "break" => Break,
        "continue" => Continue,
        "match" => Match,
        "return" => Return,
        "yield" => Yield,
        "where" => Where,
        "const" => Const,
        "let" => Let,
        "mut" => Mut,
        "enum" => Enum,
        "struct" => Struct,
        "fn" => Fn,
        "Self" => TypeSelf,
        "self" => VariableSelf,
        "use" => Use,
        "super" => Super,
        "pub" => Public,
        "as" => As,
        "package" => Package,
        "type" => Type,
        "trait" => Trait,
        "impl" => Impl,
    };
}

#[test]
fn singleline_comment() {
    macro_rules! assert_comments {
        ($($comment:expr => $is_document:expr,)+) => {
            $(
                assert_eq!(Lex::new($comment).next(), Some(Ok(Token::new(
                    Span::new(0, $comment.len()),
                    TokenKind::Comment(Comment::new_singleline($is_document))
                ))));
            )+
        };
    }

    assert_comments! {
        "//" => false,
        "///" => true,
        "//hello" => false,
        "///hello" => true,
    }
}
