#[allow(non_camel_case_types)]
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SyntaxKind {
    // Tokens
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
    /// `->`
    HYPHEN__RIGHT_CHEVRON,

    FN,
    LET,
    TRUE,
    FALSE,
    E,

    // Nodes
    Root,

    Definition,
    FunctionDefinition,

    Type,
    PathType,

    Statement,
    DefinitionStatement,
    ExpressionStatement,
    LetStatement,
    SemicolonStatement,

    Expression,
    AssignmentExpression,
    BlockExpression,
    LetExpression,
    LiteralExpression,

    FunctionParameter,

    Literal,
    ArrayLiteral,
    BooleanLiteral,
    CharLiteral,
    NumericLiteral,
    StringLiteral,
    CharLiteralFragment,
    CharLiteralEscapeSequence,
    IntegerPart,
    FractionPart,
    Exponent,
    NumberEncoding,
    NumberSign,
    NumberFragment,
    Binary,
    Octal,
    Hexadecimal,
    StringLiteralFragment,
    Escape,
    Interpolation,

    Name,
    Identifier,

    Raw,
}
