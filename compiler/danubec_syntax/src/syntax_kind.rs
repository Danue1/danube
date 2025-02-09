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
    /// `::`
    COLON__COLON,

    // Assignment operators
    /// `+=`
    PLUS__EQUAL,
    /// `+|=`
    PLUS__PIPE__EQUAL,
    /// `+%=`
    PLUS__PERCENT__EQUAL,
    /// `-=`
    HYPHEN__EQUAL,
    /// `-|=`
    HYPHEN__PIPE__EQUAL,
    /// `-%=`
    HYPHEN__PERCENT__EQUAL,
    /// `*=`
    ASTERISK__EQUAL,
    /// `*|=`
    ASTERISK__PIPE__EQUAL,
    /// `*%=`
    ASTERISK__PERCENT__EQUAL,
    /// `**=`
    ASTERISK__ASTERISK__EQUAL,
    /// `**|=`
    ASTERISK__ASTERISK__PIPE__EQUAL,
    /// `**%=`
    ASTERISK__ASTERISK__PERCENT__EQUAL,
    /// `/=`
    SLASH__EQUAL,
    /// `%=`
    PERCENT__EQUAL,
    /// `^=`
    CARET__EQUAL,
    /// `&=`
    AMPERSAND__EQUAL,
    /// `&&=`
    AMPERSAND__AMPERSAND__EQUAL,
    /// `|=`
    PIPE__EQUAL,
    /// `||=`
    PIPE__PIPE__EQUAL,
    /// `<<=`
    LEFT_CHEVRON__LEFT_CHEVRON__EQUAL,
    /// `<<|=`
    LEFT_CHEVRON__LEFT_CHEVRON__PIPE__EQUAL,
    /// `>>=`
    RIGHT_CHEVRON__RIGHT_CHEVRON__EQUAL,
    /// `>>>=`
    RIGHT_CHEVRON__RIGHT_CHEVRON__RIGHT_CHEVRON__EQUAL,

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
    /// `pub`
    PUB,
    /// `crate`
    CRATE,
    /// `super`
    SUPER,
    /// `in`
    IN,

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

    // Literal,
    ArrayLiteral,
    BooleanLiteral,
    CharLiteral,
    StringLiteral,
    BinaryNumericLiteral,
    OctalNumericLiteral,
    DecimalNumericLiteral,
    HexNumericLiteral,
    IntegerPart,
    FractionPart,
    Exponent,
    NumericFragment,
    Escape,
    Interpolation,
    ArrayLiteralElement,

    Visibility,
    VisibilityCrate,
    VisibilitySuper,
    VisibilityIn,

    Identifier,
    AssignmentOperator,

    Raw,
    Trivia,
    Error,
}
