mod context;

use context::Context;
use danubec_syntax_kind::SyntaxKind;

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
pub struct Lex;

impl Lex {
    pub fn lex(source: &str) -> Vec<(SyntaxKind, String)> {
        let mut context = Context::new(source);

        while let Some(c) = context.peek() {
            if let Some(kind) = SyntaxKind::from_char(c) {
                context.bump(kind);
                continue;
            }

            match c {
                'a'..='z' | 'A'..='Z' | '_' => {
                    let start = context.index();
                    context.advanced();
                    while expect!(context, 'a'..='z' | 'A'..='Z' | '_' | '0'..='9') {
                        //
                    }
                    let end = context.index();
                    let kind = match SyntaxKind::from_keyword(&context.slice(start..end)) {
                        Some(kind) => kind,
                        None => SyntaxKind::IDENT_KEYWORD,
                    };
                    context.bump_with(kind, start..end);
                }
                '0'..='9' => {
                    let start = context.index();
                    context.advanced();
                    while expect!(context, '0'..='9') {
                        //
                    }
                    let end = context.index();
                    context.bump_with(SyntaxKind::NUMERIC, start..end);
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
                    context.bump_with(SyntaxKind::STRING, start..end);
                }
                '\'' => {
                    let start = context.index();
                    context.advanced();
                    while !expect!(context, '\'') {
                        context.advanced();
                    }
                    expect!(context, '\'');
                    let end = context.index();
                    context.bump_with(SyntaxKind::CHAR, start..end);
                }
                _ => context.bump(SyntaxKind::ERROR),
            }
        }

        context.build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let tokens = Lex::lex("");
        assert_eq!(tokens, vec![]);
    }

    #[test]
    fn test_whitespace() {
        let tokens = Lex::lex(" \t\n");
        assert_eq!(
            tokens,
            vec![
                (SyntaxKind::WHITESPACE, " ".to_owned()),
                (SyntaxKind::TAB, "\t".to_owned()),
                (SyntaxKind::NEW_LINE, "\n".to_owned()),
            ],
        );
    }

    #[test]
    fn test_special() {
        let tokens = Lex::lex("(){}[]<>,.:;?!/\\|&^~+-*%#=");
        assert_eq!(
            tokens,
            vec![
                (SyntaxKind::LEFT_PAREN, "(".to_owned()),
                (SyntaxKind::RIGHT_PAREN, ")".to_owned()),
                (SyntaxKind::LEFT_BRACE, "{".to_owned()),
                (SyntaxKind::RIGHT_BRACE, "}".to_owned()),
                (SyntaxKind::LEFT_BRACKET, "[".to_owned()),
                (SyntaxKind::RIGHT_BRACKET, "]".to_owned()),
                (SyntaxKind::LEFT_CHEVRON, "<".to_owned()),
                (SyntaxKind::RIGHT_CHEVRON, ">".to_owned()),
                (SyntaxKind::COMMA, ",".to_owned()),
                (SyntaxKind::DOT, ".".to_owned()),
                (SyntaxKind::COLON, ":".to_owned()),
                (SyntaxKind::SEMICOLON, ";".to_owned()),
                (SyntaxKind::QUESTION, "?".to_owned()),
                (SyntaxKind::EXCLAMATION, "!".to_owned()),
                (SyntaxKind::SLASH, "/".to_owned()),
                (SyntaxKind::BACKSLASH, "\\".to_owned()),
                (SyntaxKind::PIPE, "|".to_owned()),
                (SyntaxKind::AMPERSAND, "&".to_owned()),
                (SyntaxKind::CARET, "^".to_owned()),
                (SyntaxKind::TILDE, "~".to_owned()),
                (SyntaxKind::PLUS, "+".to_owned()),
                (SyntaxKind::MINUS, "-".to_owned()),
                (SyntaxKind::ASTERISK, "*".to_owned()),
                (SyntaxKind::PERCENT, "%".to_owned()),
                (SyntaxKind::HASH, "#".to_owned()),
                (SyntaxKind::EQUAL, "=".to_owned()),
            ]
        );
    }

    #[test]
    fn test_ident() {
        let tokens = Lex::lex("hello world");
        assert_eq!(
            tokens,
            vec![
                (SyntaxKind::IDENT_KEYWORD, "hello".to_owned()),
                (SyntaxKind::WHITESPACE, " ".to_owned()),
                (SyntaxKind::IDENT_KEYWORD, "world".to_owned()),
            ],
        );
    }

    #[test]
    fn test_number() {
        let tokens = Lex::lex("123 456.789");
        assert_eq!(
            tokens,
            vec![
                (SyntaxKind::NUMERIC, "123".to_owned()),
                (SyntaxKind::WHITESPACE, " ".to_owned()),
                (SyntaxKind::NUMERIC, "456".to_owned()),
                (SyntaxKind::DOT, ".".to_owned()),
                (SyntaxKind::NUMERIC, "789".to_owned()),
            ],
        );
    }

    #[test]
    fn test_string() {
        let tokens = Lex::lex("\"hello\" \"world\"");
        assert_eq!(
            tokens,
            vec![
                (SyntaxKind::STRING, "\"hello\"".to_owned()),
                (SyntaxKind::WHITESPACE, " ".to_owned()),
                (SyntaxKind::STRING, "\"world\"".to_owned()),
            ],
        );
    }

    #[test]
    fn test_char() {
        let tokens = Lex::lex("'a' 'b'");
        assert_eq!(
            tokens,
            vec![
                (SyntaxKind::CHAR, "'a'".to_owned()),
                (SyntaxKind::WHITESPACE, " ".to_owned()),
                (SyntaxKind::CHAR, "'b'".to_owned()),
            ],
        );
    }

    #[test]
    fn test_unknown() {
        let tokens = Lex::lex("🤔");
        assert_eq!(tokens, vec![(SyntaxKind::ERROR, "🤔".to_owned())]);
    }
}
