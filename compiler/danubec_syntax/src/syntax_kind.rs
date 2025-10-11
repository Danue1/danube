#[allow(non_camel_case_types)]
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SyntaxKind {
    /// The end of the file
    END_OF_FILE,

    RAW_IDENTIFIER_START,
    IDENTIFIER,
    IDENTIFIER_SEGMENT,
    INTEGER_SEGMENT,
    FRACTION_START,
    FRACTION_SEGMENT,
    EXPONENT_START,
    EXPONENT_SIGN,
    EXPONENT_SEGMENT,
    NUMERIC_SEPARATOR,
    BINARY_START,
    BINARY_SEGMENT,
    OCTAL_START,
    OCTAL_SEGMENT,
    HEX_START,
    HEX_SEGMENT,
    CHARACTER_START,
    CHARACTER_SEGMENT,
    CHARACTER_END,
    ESCAPE_START,
    ESCAPE_SEGMENT,
    UNICODE_START,
    UNICODE_SEGMENT,
    UNICODE_END,
    STRING_START,
    STRING_SEGMENT,
    STRING_END,
    INTERPOLATION_START,
    INTERPOLATION_END,
    RAW_STRING_START,
    RAW_STRING_SEGMENT,
    RAW_STRING_END,
    LINE_COMMENT_START,
    LINE_COMMENT_SEGMENT,
    DOC_COMMENT_START,
    DOC_COMMENT_SEGMENT,

    /// ` `
    WHITESPACE,
    /// `\n`
    NEW_LINE,
    /// `\t`
    TAB,
    /// '-'
    HYPHEN,
    /// ','
    COMMA,
    /// ';'
    SEMICOLON,
    /// ':'
    COLON,
    /// '!'
    EXCLAMATION,
    /// '?'
    QUESTION,
    /// '.'
    DOT,
    /// '('
    LEFT_PAREN,
    /// ')'
    RIGHT_PAREN,
    /// '['
    LEFT_BRACKET,
    /// ']'
    RIGHT_BRACKET,
    /// '{'
    LEFT_BRACE,
    /// '}'
    RIGHT_BRACE,
    /// '@'
    AT,
    /// '*'
    ASTERISK,
    /// '/'
    SLASH,
    /// '&'
    AMPERSAND,
    /// '#'
    HASH,
    /// '%'
    PERCENT,
    /// '^'
    CARET,
    /// '+'
    PLUS,
    /// '<'
    LEFT_CHEVRON,
    /// '='
    EQUAL,
    /// '>'
    RIGHT_CHEVRON,
    /// '|'
    PIPE,
    /// '~'
    TILDE,

    /// `_`
    PLACEHOLDER,
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
    /// `mut`
    MUT,
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

    /// Anything unexpected
    ERROR,

    // Tokens in the parser

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

    /// Any node that represents an error in the syntax tree
    ERROR_NODE,

    // A source file in a krate.
    ROOT_NODE,

    /// Comments and other non-semantic tokens
    TRIVIA_NODE,

    RAW_NODE,

    TOP_LEVEL_ATTRIBUTE_NODE,
    ATTRIBUTE_NODE,
    ATTRIBUTE_ARGUMENT_NODE,
    EXPRESSION_ATTRIBUTE_ARGUMENT_NODE,
    KEY_VALUE_ATTRIBUTE_ARGUMENT_NODE,
    NESTED_ATTRIBUTE_ARGUMENT_NODE,

    DEFINITION_NODE,
    FUNCTION_DEFINITION_NODE,
    TYPE_DEFINITION_NODE,
    STRUCT_DEFINITION_NODE,
    ENUM_DEFINITION_NODE,
    TRAIT_DEFINITION_NODE,
    IMPLEMENT_DEFINITION_NODE,
    CONSTANT_DEFINITION_NODE,
    STATIC_DEFINITION_NODE,
    USE_DEFINITION_NODE,
    MODULE_DEFINITION_NODE,
    MODULE_DEFINITION_INLINE_NODE,
    MODULE_DEFINITION_EXTERNAL_NODE,
    FUNCTION_BODY_BLOCK_NODE,
    FUNCTION_BODY_UNIT_NODE,

    NEVER_TYPE_NODE,
    MUTABLE_TYPE_NODE,
    PATH_TYPE_NODE,
    SLICE_TYPE_NODE,
    TUPLE_TYPE_NODE,

    DEFINITION_STATEMENT_NODE,
    EXPRESSION_STATEMENT_NODE,
    LET_STATEMENT_NODE,
    SEMICOLON_STATEMENT_NODE,

    // Top-level expressions
    BREAK_EXPRESSION_NODE,
    CONTINUE_EXPRESSION_NODE,
    FOR_EXPRESSION_NODE,
    IF_EXPRESSION_NODE,
    LET_EXPRESSION_NODE,
    LOOP_EXPRESSION_NODE,
    MATCH_EXPRESSION_NODE,
    RETURN_EXPRESSION_NODE,
    WHILE_EXPRESSION_NODE,
    // Innermost expressions
    ARRAY_EXPRESSION_NODE,
    TUPLE_EXPRESSION_NODE,
    BLOCK_EXPRESSION_NODE,
    LITERAL_EXPRESSION_NODE,
    PATH_EXPRESSION_NODE,
    UNARY_EXPRESSION_NODE,
    // Infix expressions
    ASSIGNMENT_EXPRESSION_NODE,
    AWAIT_EXPRESSION_NODE,
    BINARY_EXPRESSION_NODE,
    FUNCTION_CALL_EXPRESSION_NODE,
    FIELD_EXPRESSION_NODE,
    INDEX_EXPRESSION_NODE,
    RANGE_FROM_TO_EXPRESSION_NODE,
    RANGE_FROM_EXPRESSION_NODE,
    RANGE_TO_EXPRESSION_NODE,
    RANGE_FULL_EXPRESSION_NODE,
    RANGE_FROM_TO_INCLUSIVE_EXPRESSION_NODE,
    RANGE_TO_INCLUSIVE_EXPRESSION_NODE,
    STRUCT_EXPRESSION_NODE,
    TRY_EXPRESSION_NODE,
    YIELD_EXPRESSION_NODE,
    METHOD_CALL_EXPRESSION_NODE,

    PATTERN_NODE,
    NEVER_PATTERN_NODE,
    PLACEHOLDER_PATTERN_NODE,
    PATH_PATTERN_NODE,
    MUTABLE_PATTERN_NODE,
    TUPLE_PATTERN_NODE,
    ARRAY_PATTERN_NODE,
    LITERAL_PATTERN_NODE,
    RANGE_FROM_TO_PATTERN_NODE,
    RANGE_FROM_TO_INCLUSIVE_PATTERN_NODE,
    RANGE_FROM_PATTERN_NODE,
    RANGE_TO_PATTERN_NODE,
    RANGE_TO_INCLUSIVE_PATTERN_NODE,
    AT_PATTERN_NODE,
    OR_PATTERN_NODE,
    NAMED_PATTERN_NODE,
    UNNAMED_PATTERN_NODE,

    FUNCTION_PARAMETER_NODE,

    // LITERAL_NODE,
    ARRAY_LITERAL_NODE,
    BOOLEAN_LITERAL_NODE,
    CHARACTER_LITERAL_NODE,
    CHARACTER_LITERAL_ONE_NODE,
    CHARACTER_LITERAL_ESCAPE_NODE,
    CHARACTER_LITERAL_UNICODE_NODE,
    STRING_LITERAL_NODE,
    STRING_LITERAL_TEXT_NODE,
    STRING_LITERAL_ESCAPE_NODE,
    STRING_LITERAL_UNICODE_NODE,
    STRING_LITERAL_INTERPOLATION_NODE,
    INTEGER_LITERAL_NODE,
    FLOAT_LITERAL_NODE,
    BINARY_NUMERIC_LITERAL_NODE,
    OCTAL_NUMERIC_LITERAL_NODE,
    HEX_NUMERIC_LITERAL_NODE,
    ESCAPE_NODE,
    INTERPOLATION_NODE,
    ARRAY_EXPRESSION_ELEMENT_NODE,
    TUPLE_TYPE_ELEMENT_NODE,
    /// `0b` | `0B` | `0o` | `0O` | `0x` | `0X`
    NUMERIC_LITERAL_PREFIX_NODE,
    /// `e` | `E`
    E_NODE,

    VISIBILITY_NODE,
    VISIBILITY_CRATE_NODE,
    VISIBILITY_SUPER_NODE,
    VISIBILITY_IN_NODE,

    IDENTIFIER_NODE,
    TARGET_TYPE_NODE,
    ASSOCIATED_DEFINITION_NODE,
    PATH_NODE,
    PATH_SEGMENT_ROOT_NODE,
    PATH_SEGMENT_SELF_NODE,
    PATH_SEGMENT_SUPER_NODE,
    PATH_SEGMENT_KRATE_NODE,
    PATH_SEGMENT_IDENTIFIER_NODE,
    ASSIGNMENT_OPERATOR_NODE,
    BINARY_OPERATOR_NODE,
    UNARY_OPERATOR_NODE,
    RANGE_OPERATOR_NODE,
    TYPE_ARGUMENT_NODE,
    TYPE_PARAMETER_NODE,
    TYPE_PARAMETER_CONSTRAINT_NODE,
    TYPE_BOUND_NODE,
    WHERE_CLAUSE_NODE,
    ENUM_VARIANT_NODE,
    ENUM_VARIANT_UNIT_NODE,
    ENUM_VARIANT_SCALAR_NODE,
    ENUM_VARIANT_NAMED_NODE,
    ENUM_VARIANT_NAMED_FIELD_NODE,
    ENUM_VARIANT_UNNAMED_NODE,
    ENUM_VARIANT_UNNAMED_FIELD_NODE,
    ENUM_VARIANT_SEQUENCE_NODE,
    STRUCT_BODY_UNIT_NODE,
    STRUCT_BODY_NAMED_NODE,
    STRUCT_BODY_NAMED_FIELD_NODE,
    STRUCT_BODY_UNNAMED_NODE,
    STRUCT_BODY_UNNAMED_FIELD_NODE,
    USE_TREE_NODE,
    USE_TREE_NESTED_NODE,
    USE_TREE_GLOB_NODE,
    USE_TREE_ELEMENT_NODE,
    USE_TREE_RENAME_NODE,
    NAMED_PATTERN_FIELD_NODE,
    ELSE_BRANCH_NODE,
    MATCH_ARM_NODE,
    FOR_ITERATOR_NODE,
    BINARY_EXPRESSION_RHS_NODE,
    OR_PATTERN_RHS_NODE,
    ARGUMENT_NODE,
    STRUCT_EXPRESSION_FIELD_NODE,
    INDEX_ELEMENT_NODE,
}

impl SyntaxKind {
    #[inline]
    pub const fn at_trivia(self) -> bool {
        use SyntaxKind::*;

        matches!(
            self,
            LINE_COMMENT_START | LINE_COMMENT_SEGMENT | WHITESPACE | NEW_LINE | TAB
        )
    }

    #[inline]
    pub const fn at_literal(self) -> bool {
        use SyntaxKind::*;

        matches!(
            self,
            CHARACTER_START
                | STRING_START
                | RAW_STRING_START
                | INTEGER_SEGMENT
                | BINARY_START
                | OCTAL_START
                | HEX_START
                | TRUE
                | FALSE
        )
    }

    #[inline]
    pub const fn at_identifier(self) -> bool {
        use SyntaxKind::*;

        matches!(self, IDENTIFIER | RAW_IDENTIFIER_START)
    }

    #[inline]
    pub const fn at_assign_operator(self) -> bool {
        use SyntaxKind::*;

        matches!(
            self,
            EQUAL
                | PLUS__EQUAL
                | PLUS__PIPE__EQUAL
                | PLUS__PERCENT__EQUAL
                | HYPHEN__EQUAL
                | HYPHEN__PIPE__EQUAL
                | HYPHEN__PERCENT__EQUAL
                | ASTERISK__EQUAL
                | ASTERISK__PIPE__EQUAL
                | ASTERISK__PERCENT__EQUAL
                | ASTERISK__ASTERISK__EQUAL
                | ASTERISK__ASTERISK__PIPE__EQUAL
                | ASTERISK__ASTERISK__PERCENT__EQUAL
                | SLASH__EQUAL
                | PERCENT__EQUAL
                | CARET__EQUAL
                | AMPERSAND__EQUAL
                | AMPERSAND__AMPERSAND__EQUAL
                | PIPE__EQUAL
                | PIPE__PIPE__EQUAL
                | LEFT_CHEVRON__LEFT_CHEVRON__EQUAL
                | LEFT_CHEVRON__LEFT_CHEVRON__PIPE__EQUAL
                | RIGHT_CHEVRON__RIGHT_CHEVRON__EQUAL
                | RIGHT_CHEVRON__RIGHT_CHEVRON__RIGHT_CHEVRON__EQUAL
        )
    }

    #[inline]
    pub const fn at_binary_operator(self) -> bool {
        use SyntaxKind::*;

        matches!(
            self,
            PIPE__PIPE
                | AMPERSAND__AMPERSAND
                | PIPE
                | CARET
                | AMPERSAND
                | LEFT_CHEVRON__LEFT_CHEVRON__PIPE
                | LEFT_CHEVRON__LEFT_CHEVRON
                | LEFT_CHEVRON__EQUAL
                | LEFT_CHEVRON
                | RIGHT_CHEVRON__RIGHT_CHEVRON__RIGHT_CHEVRON
                | RIGHT_CHEVRON__RIGHT_CHEVRON
                | RIGHT_CHEVRON__EQUAL
                | PLUS__PIPE
                | PLUS__PERCENT
                | PLUS
                | HYPHEN__PERCENT
                | HYPHEN__PIPE
                | HYPHEN
                | ASTERISK__PIPE
                | ASTERISK__PERCENT
                | ASTERISK__ASTERISK__PIPE
                | ASTERISK__ASTERISK__PERCENT
                | ASTERISK__ASTERISK
                | ASTERISK
                | SLASH
                | PERCENT
                | EQUAL__EQUAL
                | EXCLAMATION__EQUAL
        )
    }

    #[inline]
    pub const fn at_unary_operator(self) -> bool {
        use SyntaxKind::*;

        matches!(self, MUT | PLUS | HYPHEN | EXCLAMATION | TILDE)
    }

    #[inline]
    pub const fn at_range_operator(self) -> bool {
        use SyntaxKind::*;

        matches!(self, DOT__DOT | DOT__DOT__EQUAL)
    }
}
