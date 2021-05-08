use danube_lex::lex;
use danube_token::{Position, Symbol, Token, TokenKind};

macro_rules! specific_size_symbol {
    ($count:expr, $($expr:expr => $ident:ident,)+) => {
        $(
            assert_eq!(
                Ok(vec![Token {
                    position: Position::new(0, $count),
                    kind: TokenKind::Symbol(Symbol::$ident)
                }]),
                lex($expr)
            );
        )+
    };
}

macro_rules! one_size_symbol {
    ($($expr:expr => $ident:ident,)+) => {
        specific_size_symbol!(1, $($expr => $ident,)+);
    };
}

macro_rules! two_size_symbol {
    ($($expr:expr => $ident:ident,)+) => {
        specific_size_symbol!(2, $($expr => $ident,)+);
    };
}

macro_rules! three_size_symbol {
    ($($expr:expr => $ident:ident,)+) => {
        specific_size_symbol!(3, $($expr => $ident,)+);
    };
}

#[test]
fn one_size_symbol() {
    one_size_symbol! {
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
    };
}

#[test]
fn two_size_symbol() {
    two_size_symbol! {
        "|>" => PipelineRightChevron,
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
    };
}

#[test]
fn three_size_symbol() {
    three_size_symbol! {
        "..=" => DotDotEq,
        "**=" => AsteriskAsteriskEq,
        "&&=" => AmpersandAmpersandEq,
        "||=" => PipelinePipelineEq,
        "<<=" => LeftChevronLeftChevronEq,
        ">>=" => RightChevronRightChevronEq,
    };
}
