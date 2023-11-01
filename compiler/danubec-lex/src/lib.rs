mod context;

use context::Context;
use danubec_token::{Token, TokenKind};

macro_rules! expect {
    ($context:ident, $pat:pat) => {
        match $context.peek() {
            Some($pat) => {
                $context.advanced();
                true
            }
            _ => false,
        }
    };
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Lex<'lex> {
    tokens: Vec<Token<'lex>>,
}

impl<'lex> Lex<'lex> {
    pub fn lex(source: &'lex str) -> Lex<'lex> {
        lex(source)
    }

    #[cfg(test)]
    fn new(tokens: Vec<Token<'lex>>) -> Self {
        Self { tokens }
    }

    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }

    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    pub fn as_slice(&self) -> &[Token<'lex>] {
        self.tokens.as_slice()
    }
}

fn lex<'lex>(source: &'lex str) -> Lex<'lex> {
    let mut context = Context::new(source);

    while let Some(c) = context.peek() {
        match c {
            ' ' | '\t' => context.bump(TokenKind::Whitespace),
            '\n' => context.bump(TokenKind::Newline),
            '(' => context.bump(TokenKind::LeftParen),
            ')' => context.bump(TokenKind::RightParen),
            '{' => context.bump(TokenKind::LeftBrace),
            '}' => context.bump(TokenKind::RightBrace),
            '[' => context.bump(TokenKind::LeftBracket),
            ']' => context.bump(TokenKind::RightBracket),
            '<' => context.bump(TokenKind::LeftChevron),
            '>' => context.bump(TokenKind::RightChevron),
            '`' => context.bump(TokenKind::Backtick),
            ',' => context.bump(TokenKind::Comma),
            '.' => context.bump(TokenKind::Dot),
            ':' => context.bump(TokenKind::Colon),
            ';' => context.bump(TokenKind::Semicolon),
            '?' => context.bump(TokenKind::Question),
            '!' => context.bump(TokenKind::Exclamation),
            '/' => context.bump(TokenKind::Slash),
            '\\' => context.bump(TokenKind::Backslash),
            '|' => context.bump(TokenKind::Pipe),
            '&' => context.bump(TokenKind::Ampersand),
            '^' => context.bump(TokenKind::Caret),
            '~' => context.bump(TokenKind::Tilde),
            '+' => context.bump(TokenKind::Plus),
            '-' => context.bump(TokenKind::Minus),
            '*' => context.bump(TokenKind::Asterisk),
            '%' => context.bump(TokenKind::Percent),
            '#' => context.bump(TokenKind::Hash),
            '=' => context.bump(TokenKind::Equals),
            'a'..='z' | 'A'..='Z' | '_' => {
                let start = context.index();
                context.advanced();
                while expect!(context, 'a'..='z' | 'A'..='Z' | '_' | '0'..='9') {
                    //
                }
                let end = context.index();
                context.bump_with(TokenKind::Ident, start..end);
            }
            '0'..='9' => {
                let start = context.index();
                context.advanced();
                while expect!(context, '0'..='9') {
                    //
                }
                let end = context.index();
                context.bump_with(TokenKind::Number, start..end);
            }
            '"' => {
                let start = context.index();
                context.advanced();
                loop {
                    match context.peek() {
                        Some('"') => {
                            context.advanced();
                            break;
                        }
                        _ => context.advanced(),
                    }
                }
                let end = context.index();
                context.bump_with(TokenKind::String, start..end);
            }
            '\'' => {
                let start = context.index();
                context.advanced();
                while !expect!(context, '\'') {
                    context.advanced();
                }
                expect!(context, '\'');
                let end = context.index();
                context.bump_with(TokenKind::Char, start..end);
            }
            _ => context.bump(TokenKind::Unknown),
        }
    }

    Lex {
        tokens: context.build(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let tokens = lex("");
        assert_eq!(
            tokens,
            Lex::new(vec![Token {
                kind: TokenKind::Eof,
                source: "",
            }]),
        );
    }

    #[test]
    fn test_whitespace() {
        let tokens = lex(" \t\n");
        assert_eq!(
            tokens,
            Lex::new(vec![
                Token {
                    kind: TokenKind::Whitespace,
                    source: " ",
                },
                Token {
                    kind: TokenKind::Whitespace,
                    source: "\t",
                },
                Token {
                    kind: TokenKind::Newline,
                    source: "\n",
                },
                Token {
                    kind: TokenKind::Eof,
                    source: "",
                },
            ]),
        );
    }

    #[test]
    fn test_special() {
        let tokens = lex("(){}[]<>,.:;?!/\\|&^~+-*%#=");
        assert_eq!(
            tokens,
            Lex::new(vec![
                Token {
                    kind: TokenKind::LeftParen,
                    source: "(",
                },
                Token {
                    kind: TokenKind::RightParen,
                    source: ")",
                },
                Token {
                    kind: TokenKind::LeftBrace,
                    source: "{",
                },
                Token {
                    kind: TokenKind::RightBrace,
                    source: "}",
                },
                Token {
                    kind: TokenKind::LeftBracket,
                    source: "[",
                },
                Token {
                    kind: TokenKind::RightBracket,
                    source: "]",
                },
                Token {
                    kind: TokenKind::LeftChevron,
                    source: "<",
                },
                Token {
                    kind: TokenKind::RightChevron,
                    source: ">",
                },
                Token {
                    kind: TokenKind::Comma,
                    source: ",",
                },
                Token {
                    kind: TokenKind::Dot,
                    source: ".",
                },
                Token {
                    kind: TokenKind::Colon,
                    source: ":",
                },
                Token {
                    kind: TokenKind::Semicolon,
                    source: ";",
                },
                Token {
                    kind: TokenKind::Question,
                    source: "?",
                },
                Token {
                    kind: TokenKind::Exclamation,
                    source: "!",
                },
                Token {
                    kind: TokenKind::Slash,
                    source: "/",
                },
                Token {
                    kind: TokenKind::Backslash,
                    source: "\\",
                },
                Token {
                    kind: TokenKind::Pipe,
                    source: "|",
                },
                Token {
                    kind: TokenKind::Ampersand,
                    source: "&",
                },
                Token {
                    kind: TokenKind::Caret,
                    source: "^",
                },
                Token {
                    kind: TokenKind::Tilde,
                    source: "~",
                },
                Token {
                    kind: TokenKind::Plus,
                    source: "+",
                },
                Token {
                    kind: TokenKind::Minus,
                    source: "-",
                },
                Token {
                    kind: TokenKind::Asterisk,
                    source: "*",
                },
                Token {
                    kind: TokenKind::Percent,
                    source: "%",
                },
                Token {
                    kind: TokenKind::Hash,
                    source: "#",
                },
                Token {
                    kind: TokenKind::Equals,
                    source: "=",
                },
                Token {
                    kind: TokenKind::Eof,
                    source: "",
                },
            ]),
        );
    }

    #[test]
    fn test_ident() {
        let tokens = lex("hello world");
        assert_eq!(
            tokens,
            Lex::new(vec![
                Token {
                    kind: TokenKind::Ident,
                    source: "hello",
                },
                Token {
                    kind: TokenKind::Whitespace,
                    source: " ",
                },
                Token {
                    kind: TokenKind::Ident,
                    source: "world",
                },
                Token {
                    kind: TokenKind::Eof,
                    source: "",
                },
            ]),
        );
    }

    #[test]
    fn test_number() {
        let tokens = lex("123 456.789");
        assert_eq!(
            tokens,
            Lex::new(vec![
                Token {
                    kind: TokenKind::Number,
                    source: "123",
                },
                Token {
                    kind: TokenKind::Whitespace,
                    source: " ",
                },
                Token {
                    kind: TokenKind::Number,
                    source: "456",
                },
                Token {
                    kind: TokenKind::Dot,
                    source: ".",
                },
                Token {
                    kind: TokenKind::Number,
                    source: "789",
                },
                Token {
                    kind: TokenKind::Eof,
                    source: "",
                },
            ]),
        );
    }

    #[test]
    fn test_string() {
        let tokens = lex("\"hello\" \"world\"");
        assert_eq!(
            tokens,
            Lex::new(vec![
                Token {
                    kind: TokenKind::String,
                    source: "\"hello\"",
                },
                Token {
                    kind: TokenKind::Whitespace,
                    source: " ",
                },
                Token {
                    kind: TokenKind::String,
                    source: "\"world\"",
                },
                Token {
                    kind: TokenKind::Eof,
                    source: "",
                },
            ]),
        );
    }

    #[test]
    fn test_char() {
        let tokens = lex("'a' 'b'");
        assert_eq!(
            tokens,
            Lex::new(vec![
                Token {
                    kind: TokenKind::Char,
                    source: "'a'",
                },
                Token {
                    kind: TokenKind::Whitespace,
                    source: " ",
                },
                Token {
                    kind: TokenKind::Char,
                    source: "'b'",
                },
                Token {
                    kind: TokenKind::Eof,
                    source: "",
                },
            ]),
        );
    }

    #[test]
    fn test_unknown() {
        let tokens = lex("🤔");
        assert_eq!(
            tokens,
            Lex::new(vec![
                Token {
                    kind: TokenKind::Unknown,
                    source: "🤔",
                },
                Token {
                    kind: TokenKind::Eof,
                    source: "",
                },
            ]),
        );
    }
}
