pub const EOF: Token<'static> = Token {
    kind: TokenKind::Eof,
    source: "",
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Token<'token> {
    pub kind: TokenKind,
    pub source: &'token str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenKind {
    Eof,
    Newline,
    Whitespace,
    LeftParen,    // (
    RightParen,   // )
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    LeftChevron,  // <
    RightChevron, // >
    Backtick,     // `
    Comma,        // ,
    Dot,          // .
    Colon,        // :
    Semicolon,    // ;
    Question,     // ?
    Exclamation,  // !
    Slash,        // /
    Backslash,    // \
    Pipe,         // |
    Ampersand,    // &
    Caret,        // ^
    Tilde,        // ~
    Plus,         // +
    Minus,        // -
    Asterisk,     // *
    Percent,      // %
    Hash,         // #
    Equals,       // =
    Ident,        // hello
    Number,       // 123
    String,       // "hello"
    Char,         // 'a'
    Unknown,
}

impl<'token> Token<'token> {
    pub const fn len(&self) -> usize {
        self.source.len()
    }
}
