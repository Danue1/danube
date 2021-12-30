#[macro_use]
extern crate lazy_static;

pub mod keyword;
pub mod symbol;

use danube_span::Span;
pub use keyword::*;
pub use symbol::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub const fn new(span: Span, kind: TokenKind) -> Self {
        Token { span, kind }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    // 1
    LeftParens,   // (
    RightParens,  // )
    LeftBracket,  // [
    RightBracket, // ]
    LeftBrace,    // {
    RightBrace,   // }
    LeftChevron,  // <
    RightChevron, // >
    Hash,         // #
    Dot,          // .
    Comma,        // ,
    Colon,        // :
    Semicolon,    // ;
    Eq,           // =
    Plus,         // +
    Hyphen,       // -
    Asterisk,     // *
    Slash,        // /
    Percent,      // %
    Exclamation,  // !
    Question,     // ?
    Ampersand,    // &
    Pipeline,     // |
    Tilde,        // ~
    Caret,        // ^
    Underscore,   // _

    // 2
    HyphenRightChevron,       // ->
    EqRightChevron,           // =>
    DotDot,                   // ..
    ColonColon,               // ::
    EqEq,                     // ==
    ExclamationEq,            // !=
    PlusEq,                   // +=
    HyphenEq,                 // -=
    AsteriskEq,               // *=
    SlashEq,                  // /=
    PercentEq,                // %=
    AsteriskAsterisk,         // **
    AmpersandAmpersand,       // &&
    PipelinePipeline,         // ||
    AmpersandEq,              // &=
    PipelineEq,               // |=
    TildeEq,                  // ~=
    CaretEq,                  // ^=
    LeftChevronLeftChevron,   // <<
    RightChevronRightChevron, // >>
    LeftChevronEq,            // <=
    RightChevronEq,           // >=

    // 3
    DotDotEq,                   // ..=
    AsteriskAsteriskEq,         // **=
    AmpersandAmpersandEq,       // &&=
    PipelinePipelineEq,         // ||=
    LeftChevronLeftChevronEq,   // <<=
    RightChevronRightChevronEq, // >>=

    Identifier(Symbol),
    Literal(Symbol, LiteralKind),
    Comment(Symbol),
}

#[derive(Debug, PartialEq, Clone)]
pub enum LiteralKind {
    Char,
    Integer,
    Float,
    String,
}

impl TokenKind {
    pub const fn count(&self) -> Option<usize> {
        let count = match self {
            // 1
            TokenKind::LeftParens => 1,   // (
            TokenKind::RightParens => 1,  // )
            TokenKind::LeftBracket => 1,  // [
            TokenKind::RightBracket => 1, // ]
            TokenKind::LeftBrace => 1,    // {
            TokenKind::RightBrace => 1,   // }
            TokenKind::LeftChevron => 1,  // <
            TokenKind::RightChevron => 1, // >
            TokenKind::Hash => 1,         // #
            TokenKind::Dot => 1,          // .
            TokenKind::Comma => 1,        // ,
            TokenKind::Colon => 1,        // :
            TokenKind::Semicolon => 1,    // ;
            TokenKind::Eq => 1,           // =
            TokenKind::Plus => 1,         // +
            TokenKind::Hyphen => 1,       // -
            TokenKind::Asterisk => 1,     // *
            TokenKind::Slash => 1,        // /
            TokenKind::Percent => 1,      // %
            TokenKind::Exclamation => 1,  // !
            TokenKind::Question => 1,     // ?
            TokenKind::Ampersand => 1,    // &
            TokenKind::Pipeline => 1,     // |
            TokenKind::Tilde => 1,        // ~
            TokenKind::Caret => 1,        // ^
            TokenKind::Underscore => 1,   // _

            // 2
            TokenKind::HyphenRightChevron => 2,       // ->
            TokenKind::EqRightChevron => 2,           // =>
            TokenKind::DotDot => 2,                   // ..
            TokenKind::ColonColon => 2,               // ::
            TokenKind::EqEq => 2,                     // ==
            TokenKind::ExclamationEq => 2,            // !=
            TokenKind::PlusEq => 2,                   // +=
            TokenKind::HyphenEq => 2,                 // -=
            TokenKind::AsteriskEq => 2,               // *=
            TokenKind::SlashEq => 2,                  // /=
            TokenKind::PercentEq => 2,                // %=
            TokenKind::AsteriskAsterisk => 2,         // **
            TokenKind::AmpersandAmpersand => 2,       // &&
            TokenKind::PipelinePipeline => 2,         // ||
            TokenKind::AmpersandEq => 2,              // &=
            TokenKind::PipelineEq => 2,               // |=
            TokenKind::TildeEq => 2,                  // ~=
            TokenKind::CaretEq => 2,                  // ^=
            TokenKind::LeftChevronLeftChevron => 2,   // <<
            TokenKind::RightChevronRightChevron => 2, // >>
            TokenKind::LeftChevronEq => 2,            // <=
            TokenKind::RightChevronEq => 2,           // >=

            // 3
            TokenKind::DotDotEq => 3,                   // ..=
            TokenKind::AsteriskAsteriskEq => 3,         // **=
            TokenKind::AmpersandAmpersandEq => 3,       // &&=
            TokenKind::PipelinePipelineEq => 3,         // ||=
            TokenKind::LeftChevronLeftChevronEq => 3,   // <<=
            TokenKind::RightChevronRightChevronEq => 3, // >>=

            _ => return None,
        };

        Some(count)
    }
}
