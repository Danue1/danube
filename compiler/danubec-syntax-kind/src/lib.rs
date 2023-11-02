use SyntaxKind::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
#[repr(u16)]
pub enum SyntaxKind {
    EOF,
    ERROR,

    // Punctuation
    NEW_LINE,      // \n
    WHITESPACE,    // ' '
    TAB,           // \t
    LEFT_PAREN,    // (
    RIGHT_PAREN,   // )
    LEFT_BRACE,    // {
    RIGHT_BRACE,   // }
    LEFT_BRACKET,  // [
    RIGHT_BRACKET, // ]
    LEFT_CHEVRON,  // <
    RIGHT_CHEVRON, // >
    BACKTICK,      // `
    COMMA,         // ,
    DOT,           // .
    COLON,         // :
    SEMICOLON,     // ;
    QUESTION,      // ?
    EXCLAMATION,   // !
    SLASH,         // /
    BACKSLASH,     // \
    PIPE,          // |
    AMPERSAND,     // &
    CARET,         // ^
    TILDE,         // ~
    PLUS,          // +
    MINUS,         // -
    ASTERISK,      // *
    PERCENT,       // %
    HASH,          // #
    EQUAL,         // =

    // Literals
    NUMBER, // 123
    STRING, // "hello"
    CHAR,   // 'a'

    // Keyword
    IDENT_KEYWORD,         // hello
    IF_KEYWORD,            // if
    ELSE_KEYWORD,          // else
    LOOP_KEYWORD,          // loop
    WHILE_KEYWORD,         // while
    FOR_KEYWORD,           // for
    IN_KEYWORD,            // in
    BREAK_KEYWORD,         // break
    CONTINUE_KEYWORD,      // continue
    RETURN_KEYWORD,        // return
    FN_KEYWORD,            // fn
    STRUCT_KEYWORD,        // struct
    ENUM_KEYWORD,          // enum
    TRAIT_KEYWORD,         // trait
    IMPL_KEYWORD,          // impl
    USE_KEYWORD,           // use
    AS_KEYWORD,            // as
    TYPE_KEYWORD,          // type
    CONST_KEYWORD,         // const
    STATIC_KEYWORD,        // static
    LET_KEYWORD,           // let
    MUT_KEYWORD,           // mut
    SELF_INSTANCE_KEYWORD, // self
    SELF_TYPE_KEYWORD,     // Self
    SUPER_KEYWORD,         // super
    PUB_KEYWORD,           // pub

    // Nodes
    AST_NODE,
    STRUCT_ITEM_NODE,
    ENUM_ITEM_NODE,
    NAMED_STRUCT_FIELDS_NODE,
    NAMED_STRUCT_FIELD_NODE,
    UNNAMED_STRUCT_FIELDS_NODE,
    UNNAMED_STRUCT_FIELD_NODE,
    TYPE_NODE,
    UNNAMED_TYPE_KIND_NODE, // (type1, type2)
    PATH_TYPE_KIND_NODE,    // foo::bar::baz
    PATH_NODE,              // foo::bar::baz
    IDENT_NODE,             // foo
    VISIBILITY_NODE,        // pub
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}

impl rowan::Language for SyntaxKind {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        unsafe { std::mem::transmute::<u16, SyntaxKind>(raw.0) }
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}

impl SyntaxKind {
    pub const fn is_whitespace(self) -> bool {
        matches!(self, NEW_LINE | WHITESPACE | TAB)
    }

    pub const fn is_punctuation(self) -> bool {
        matches!(
            self,
            NEW_LINE
                | WHITESPACE
                | LEFT_PAREN
                | RIGHT_PAREN
                | LEFT_BRACE
                | RIGHT_BRACE
                | LEFT_BRACKET
                | RIGHT_BRACKET
                | LEFT_CHEVRON
                | RIGHT_CHEVRON
                | BACKTICK
                | COMMA
                | DOT
                | COLON
                | SEMICOLON
                | QUESTION
                | EXCLAMATION
                | SLASH
                | BACKSLASH
                | PIPE
                | AMPERSAND
                | CARET
                | TILDE
                | PLUS
                | MINUS
                | ASTERISK
                | PERCENT
                | HASH
                | EQUAL
        )
    }

    pub const fn is_literal(self) -> bool {
        matches!(self, NUMBER | STRING | CHAR)
    }

    pub const fn is_keyword(self) -> bool {
        matches!(
            self,
            IDENT_KEYWORD
                | IF_KEYWORD
                | ELSE_KEYWORD
                | LOOP_KEYWORD
                | WHILE_KEYWORD
                | FOR_KEYWORD
                | IN_KEYWORD
                | BREAK_KEYWORD
                | CONTINUE_KEYWORD
                | RETURN_KEYWORD
                | FN_KEYWORD
                | STRUCT_KEYWORD
                | ENUM_KEYWORD
                | TRAIT_KEYWORD
                | IMPL_KEYWORD
                | USE_KEYWORD
                | AS_KEYWORD
                | TYPE_KEYWORD
                | CONST_KEYWORD
                | STATIC_KEYWORD
                | LET_KEYWORD
                | MUT_KEYWORD
                | SELF_INSTANCE_KEYWORD
                | SELF_TYPE_KEYWORD
                | SUPER_KEYWORD
                | PUB_KEYWORD
        )
    }

    pub fn from_char(c: char) -> Option<SyntaxKind> {
        let kind = match c {
            '\n' => NEW_LINE,
            ' ' => WHITESPACE,
            '\t' => TAB,
            '(' => LEFT_PAREN,
            ')' => RIGHT_PAREN,
            '{' => LEFT_BRACE,
            '}' => RIGHT_BRACE,
            '[' => LEFT_BRACKET,
            ']' => RIGHT_BRACKET,
            '<' => LEFT_CHEVRON,
            '>' => RIGHT_CHEVRON,
            '`' => BACKTICK,
            ',' => COMMA,
            '.' => DOT,
            ':' => COLON,
            ';' => SEMICOLON,
            '?' => QUESTION,
            '!' => EXCLAMATION,
            '/' => SLASH,
            '\\' => BACKSLASH,
            '|' => PIPE,
            '&' => AMPERSAND,
            '^' => CARET,
            '~' => TILDE,
            '+' => PLUS,
            '-' => MINUS,
            '*' => ASTERISK,
            '%' => PERCENT,
            '#' => HASH,
            '=' => EQUAL,
            _ => return None,
        };

        Some(kind)
    }

    pub fn from_keyword(ident: &str) -> Option<SyntaxKind> {
        let keyword = match ident {
            "if" => IF_KEYWORD,
            "else" => ELSE_KEYWORD,
            "loop" => LOOP_KEYWORD,
            "while" => WHILE_KEYWORD,
            "for" => FOR_KEYWORD,
            "in" => IN_KEYWORD,
            "break" => BREAK_KEYWORD,
            "continue" => CONTINUE_KEYWORD,
            "return" => RETURN_KEYWORD,
            "fn" => FN_KEYWORD,
            "struct" => STRUCT_KEYWORD,
            "enum" => ENUM_KEYWORD,
            "trait" => TRAIT_KEYWORD,
            "impl" => IMPL_KEYWORD,
            "use" => USE_KEYWORD,
            "as" => AS_KEYWORD,
            "type" => TYPE_KEYWORD,
            "const" => CONST_KEYWORD,
            "static" => STATIC_KEYWORD,
            "let" => LET_KEYWORD,
            "mut" => MUT_KEYWORD,
            "self" => SELF_INSTANCE_KEYWORD,
            "Self" => SELF_TYPE_KEYWORD,
            "super" => SUPER_KEYWORD,
            "pub" => PUB_KEYWORD,
            _ => return None,
        };

        Some(keyword)
    }
}
