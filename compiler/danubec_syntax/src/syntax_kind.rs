#[allow(non_camel_case_types)]
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SyntaxKind {
    // Tokens
    UNEXPECTED,
    /// ` `
    WHITESPACE,
    /// `\n`
    NEW_LINE,
    /// `\t`
    TAB,
    /// `~`
    TILDE,
    /// `\``
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

    /// `[a-zA-Z]+`
    ALPHABETIC,
    /// `[0-9]+`
    NUMERIC,
    /// `0b` | `0B` | `0o` | `0O` | `0x` | `0X`
    NUMERIC_LITERAL_PREFIX,

    /// `fn`
    FN,
    /// `let`
    LET,
    /// `true`
    TRUE,
    /// `false`
    FALSE,
    /// `e` | `E`
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
    BinaryNumericLiteral,
    OctalNumericLiteral,
    DecimalNumericLiteral,
    HexNumericLiteral,
    IntegerPart,
    FractionPart,
    Exponent,
    NumericFragment,
    StringLiteralFragment,
    Escape,
    Interpolation,

    Name,
    Identifier,

    Raw,
}
