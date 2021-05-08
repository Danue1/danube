#[derive(Debug, PartialEq, Clone)]
pub enum Symbol {
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

    // 2
    PipelineRightChevron,     // |>
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
}

impl Symbol {
    #[inline(always)]
    pub const fn count(&self) -> usize {
        match self {
            Symbol::LeftParens
            | Symbol::RightParens
            | Symbol::LeftBracket
            | Symbol::RightBracket
            | Symbol::LeftBrace
            | Symbol::RightBrace
            | Symbol::LeftChevron
            | Symbol::RightChevron
            | Symbol::Hash
            | Symbol::Dot
            | Symbol::Comma
            | Symbol::Colon
            | Symbol::Semicolon
            | Symbol::Eq
            | Symbol::Plus
            | Symbol::Hyphen
            | Symbol::Asterisk
            | Symbol::Slash
            | Symbol::Percent
            | Symbol::Exclamation
            | Symbol::Question
            | Symbol::Ampersand
            | Symbol::Pipeline
            | Symbol::Tilde
            | Symbol::Caret => 1,
            Symbol::PipelineRightChevron
            | Symbol::HyphenRightChevron
            | Symbol::EqRightChevron
            | Symbol::DotDot
            | Symbol::ColonColon
            | Symbol::EqEq
            | Symbol::ExclamationEq
            | Symbol::PlusEq
            | Symbol::HyphenEq
            | Symbol::AsteriskEq
            | Symbol::SlashEq
            | Symbol::PercentEq
            | Symbol::AsteriskAsterisk
            | Symbol::AmpersandAmpersand
            | Symbol::PipelinePipeline
            | Symbol::AmpersandEq
            | Symbol::PipelineEq
            | Symbol::TildeEq
            | Symbol::CaretEq
            | Symbol::LeftChevronLeftChevron
            | Symbol::RightChevronRightChevron
            | Symbol::LeftChevronEq
            | Symbol::RightChevronEq => 2,
            Symbol::DotDotEq
            | Symbol::AsteriskAsteriskEq
            | Symbol::AmpersandAmpersandEq
            | Symbol::PipelinePipelineEq
            | Symbol::LeftChevronLeftChevronEq
            | Symbol::RightChevronRightChevronEq => 3,
        }
    }
}
