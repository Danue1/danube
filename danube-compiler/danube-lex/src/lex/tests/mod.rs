use crate::Lex;
use danube_span::Span;
use danube_token::{LiteralKind, Symbol, Token, TokenKind};

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
    macro_rules! assert_integers {
        ($($integer:expr),+) => {
            $(
                assert_eq!(
                    Lex::new($integer).next(),
                    Some(Ok(Token::new(
                        Span::new(0, $integer.len()),
                        TokenKind::Literal(Symbol::intern($integer), LiteralKind::Integer),
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

            $(
                assert_eq!(
                    Lex::new($floating).next(),
                    Some(Ok(Token::new(
                        Span::new(0, $floating.len()),
                        TokenKind::Literal(Symbol::intern($floating), LiteralKind::Float),
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
        ($($char:expr => $expect:expr,)+) => {

            $(
                assert_eq!(
                    Lex::new($char).next(),
                    Some(Ok(Token::new(
                        Span::new(0, $char.len()),
                        TokenKind::Literal(Symbol::intern($expect), LiteralKind::Char),
                    )))
                );
            )+
        };
    }

    assert_chars! {
        r#"'a'"# => "a",
        r#"'\r'"# => "\r",
        r#"'\n'"# => "\n",
        r#"'\t'"# => "\t",
    };
}

#[test]
fn strings() {
    macro_rules! assert_strings {
        ($($string:expr => $expect:expr,)+) => {

            $(
                assert_eq!(
                    Lex::new($string).next(),
                    Some(Ok(Token::new(
                        Span::new(0, $string.len()),
                        TokenKind::Literal(Symbol::intern($expect), LiteralKind::String),
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
                let kind = TokenKind::Identifier(Symbol::intern($identifier));
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
                assert_eq!(
                    Lex::new($keyword).next(),
                    Some(Ok(Token::new(
                        Span::new(0, $keyword.len()),
                        TokenKind::Identifier(Symbol::intern($keyword)),
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
                assert_eq!(Lex::new($comment).next(), Some(Ok(Token::new(
                    Span::new(0, $comment.len()),
                    TokenKind::Comment(Symbol::intern($comment))
                ))));
            )+
        };
    }

    assert_comments!["//", "///", "//hello", "///hello"];
}
