#[allow(non_camel_case_types)]
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SyntaxKind {
    // Tokens in the lexer
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

    /// `[a-zA-Z]+`
    ALPHABETIC,
    /// `[0-9]+`
    NUMERIC,

    /// `fn`
    FN,
    /// `let`
    LET,
    /// `true`
    TRUE,
    /// `false`
    FALSE,
    /// `pub`
    PUB,
    /// `crate`
    CRATE,
    /// `super`
    SUPER,
    /// `in`
    IN,
    /// `type`
    TYPE,
    /// `where`
    WHERE,
    /// `struct`
    STRUCT,
    /// `enum`
    ENUM,
    /// `trait`
    TRAIT,
    /// `impl`
    IMPL,
    /// `const`
    CONST,
    /// `static`
    STATIC,
    /// `use`
    USE,
    /// `mod`
    MOD,
    /// `self`
    SELF,
    /// `Self`
    SELF_UPPERCASE,
    /// `as`
    AS,
    /// `for`
    FOR,
    /// `if`
    IF,
    /// `else`
    ELSE,
    /// `match`
    MATCH,
    /// `loop`
    LOOP,
    /// `while`
    WHILE,
    /// `return`
    RETURN,
    /// `break`
    BREAK,
    /// `continue`
    CONTINUE,
    /// `await`
    AWAIT,
    /// `yield`
    YIELD,

    UNEXPECTED,

    // Tokens in the parser
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

    // Binary operators
    /// `||`
    PIPE__PIPE,

    /// `&&`
    AMPERSAND__AMPERSAND,

    /// `==`
    EQUAL__EQUAL,
    /// `!=`
    EXCLAMATION__EQUAL,
    /// `<=`
    LEFT_CHEVRON__EQUAL,
    /// `>=`
    RIGHT_CHEVRON__EQUAL,

    /// `<<|`
    LEFT_CHEVRON__LEFT_CHEVRON__PIPE,
    /// `<<`
    LEFT_CHEVRON__LEFT_CHEVRON,
    /// `>>>`
    RIGHT_CHEVRON__RIGHT_CHEVRON__RIGHT_CHEVRON,
    /// `>>`
    RIGHT_CHEVRON__RIGHT_CHEVRON,

    /// `+|`
    PLUS__PIPE,
    /// `+%`
    PLUS__PERCENT,
    /// `-|`
    HYPHEN__PIPE,
    /// `-%`
    HYPHEN__PERCENT,

    /// `*|`
    ASTERISK__PIPE,
    /// `*%`
    ASTERISK__PERCENT,
    /// `**`
    ASTERISK__ASTERISK,
    /// `**|`
    ASTERISK__ASTERISK__PIPE,
    /// `**%`
    ASTERISK__ASTERISK__PERCENT,

    /// `..`
    DOT__DOT,
    /// `..=`
    DOT__DOT__EQUAL,
    /// `=>`
    EQUAL__RIGHT_CHEVRON,

    // Nodes in the parser
    Root,

    Definition,
    FunctionDefinition,
    TypeDefinition,
    StructDefinition,
    EnumDefinition,
    TraitDefinition,
    ImplDefinition,
    ConstDefinition,
    StaticDefinition,
    UseDefinition,
    ModuleDefinition,

    Type,
    NeverType,
    PathType,
    SliceType,
    TupleType,

    Statement,
    DefinitionStatement,
    ExpressionStatement,
    LetStatement,
    SemicolonStatement,

    Expression,
    // Top-level expressions
    BreakExpression,
    ContinueExpression,
    ForExpression,
    IfExpression,
    LetExpression,
    LoopExpression,
    MatchExpression,
    ReturnExpression,
    WhileExpression,
    // Innermost expressions
    ArrayExpression,
    BlockExpression,
    LiteralExpression,
    PathExpression,
    UnaryExpression,
    // Infix expressions
    AssignmentExpression,
    AwaitExpression,
    BinaryExpression,
    CallExpression,
    FieldExpression,
    IndexExpression,
    RangeExpression,
    StructExpression,
    TryExpression,
    YieldExpression,

    Pattern,
    NeverPattern,
    PlaceholderPattern,
    PathPattern,
    TuplePattern,
    ArrayPattern,
    LiteralPattern,
    RestPattern,
    OrPattern,
    NamedPattern,
    UnnamedPattern,

    FunctionParameter,

    // Literal,
    ArrayLiteral,
    BooleanLiteral,
    CharLiteral,
    NumericLiteral,
    StringLiteral,
    BinaryNumericLiteral,
    OctalNumericLiteral,
    DecimalNumericLiteral,
    HexNumericLiteral,
    IntegerPart,
    FractionPart,
    ExponentPart,
    ExponentPartSign,
    NumericFragment,
    Escape,
    Interpolation,
    ArrayExpressionElement,
    TupleTypeElement,
    /// `0b` | `0B` | `0o` | `0O` | `0x` | `0X`
    NUMERIC_LITERAL_PREFIX,
    /// `e` | `E`
    E,

    Visibility,
    VisibilityCrate,
    VisibilitySuper,
    VisibilityIn,

    Identifier,
    TargetType,
    AssociatedItem,
    Path,
    PathSegment,
    AssignmentOperator,
    BinaryOperator,
    UnaryOperator,
    TypeArgument,
    TypeParameter,
    TypeConstraint,
    TypeConstraintParameter,
    WhereClause,
    EnumVariant,
    EnumVariantNamed,
    EnumVariantNamedField,
    EnumVariantUnnamed,
    EnumVariantUnnamedField,
    EnumVariantSequence,
    StructBodyNamed,
    StructBodyNamedField,
    StructBodyUnnamed,
    StructBodyUnnamedField,
    UseTree,
    UseTreeBarrel,
    UseTreeIdent,
    UseTreeNested,
    TuplePatternElement,
    ArrayPatternElement,
    NamedPatternElement,
    UnnamedPatternElement,
    ElseBranch,
    MatchArm,
    ForIterator,

    Raw,
    Trivia,
    Error,
}
