#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
    If,           // if
    Else,         // else
    For,          // for
    While,        // while
    Loop,         // loop
    In,           // in
    Break,        // break
    Continue,     // continue
    Match,        // match
    Return,       // return
    Yield,        // yield
    Where,        // where
    Const,        // const
    Let,          // let
    Mut,          // mut
    Enum,         // enum
    Struct,       // struct
    Fn,           // fn
    TypeSelf,     // Self
    VariableSelf, // self
    Use,          // use
    Super,        // super
    Public,       // pub
    As,           // as
    Package,      // package
    Type,         // type
    Trait,        // trait
    Impl,         // impl
}

impl TryFrom<&str> for Keyword {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let keyword = match value {
            "if" => Keyword::If,
            "else" => Keyword::Else,
            "for" => Keyword::For,
            "while" => Keyword::While,
            "loop" => Keyword::Loop,
            "in" => Keyword::In,
            "break" => Keyword::Break,
            "continue" => Keyword::Continue,
            "match" => Keyword::Match,
            "return" => Keyword::Return,
            "yield" => Keyword::Yield,
            "where" => Keyword::Where,
            "const" => Keyword::Const,
            "let" => Keyword::Let,
            "mut" => Keyword::Mut,
            "enum" => Keyword::Enum,
            "struct" => Keyword::Struct,
            "fn" => Keyword::Fn,
            "Self" => Keyword::TypeSelf,
            "self" => Keyword::VariableSelf,
            "use" => Keyword::Use,
            "super" => Keyword::Super,
            "pub" => Keyword::Public,
            "as" => Keyword::As,
            "package" => Keyword::Package,
            "type" => Keyword::Type,
            "trait" => Keyword::Trait,
            "impl" => Keyword::Impl,
            _ => return Err(()),
        };

        Ok(keyword)
    }
}
