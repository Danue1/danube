#[allow(non_camel_case_types)]
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SyntaxKind {
    /// ` `
    WHITESPACE,
    /// `\n`
    NEW_LINE,
    /// `\t`
    TAB,
    /// `~`
    TILDE,
    /// `~`
    BACKTICK,
    /// `!`
    EXCLAMATION,
    /// `@`
    AT,
    /// `#`
    HASH,
    /// `$`
    DOLLAR,
    /// `%`
    PERCENT,
    /// `^`
    CARET,
    /// `&`
    AMPERSAND,
    /// `*`
    ASTERISK,
    /// `-`
    HYPHEN,
    /// `_`
    UNDERSCORE,
    /// `+`
    PLUS,
    /// `=`
    EQUAL,
    /// `|`
    PIPE,
    /// `:`
    COLON,
    /// `;`
    SEMICOLON,
    /// `,`
    COMMA,
    /// `.`
    DOT,
    /// `/`
    SLASH,
    /// `?`
    QUESTION,
    /// `\`
    BACKSLASH,
    /// `{`
    LEFT_BRACE,
    /// `}`
    RIGHT_BRACE,
    /// `[`
    LEFT_BRACKET,
    /// `]`
    RIGHT_BRACKET,
    /// `(`
    LEFT_PAREN,
    /// `)`
    RIGHT_PAREN,
    /// `<`
    LEFT_CHEVRON,
    /// `>`
    RIGHT_CHEVRON,
    /// `"`
    DOUBLE_QUOTE,
    /// `'`
    SINGLE_QUOTE,

    Root,
}
