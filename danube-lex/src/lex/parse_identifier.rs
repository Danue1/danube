use super::*;

#[derive(Debug, PartialEq)]
pub(super) enum Identifier {
    Unreserved(String),
    Reserved(Keyword),
}

pub(super) fn parse_identifier(s: LexSpan) -> LexResult<Identifier> {
    let (s, head) = is_a("_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ")(s)?;
    let (s, rest) = opt(is_a(
        "_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ123456789",
    ))(s)?;
    let string = format!(
        "{}{}",
        head.fragment().to_owned(),
        rest.map(|s| s.fragment().to_owned()).unwrap_or("")
    );

    let identifier = match string.as_ref() {
        "if" => Identifier::Reserved(Keyword::If),
        "else" => Identifier::Reserved(Keyword::Else),
        "for" => Identifier::Reserved(Keyword::For),
        "while" => Identifier::Reserved(Keyword::While),
        "loop" => Identifier::Reserved(Keyword::Loop),
        "in" => Identifier::Reserved(Keyword::In),
        "break" => Identifier::Reserved(Keyword::Break),
        "continue" => Identifier::Reserved(Keyword::Continue),
        "match" => Identifier::Reserved(Keyword::Match),
        "return" => Identifier::Reserved(Keyword::Return),
        "yield" => Identifier::Reserved(Keyword::Yield),
        "where" => Identifier::Reserved(Keyword::Where),
        "const" => Identifier::Reserved(Keyword::Const),
        "static" => Identifier::Reserved(Keyword::Static),
        "let" => Identifier::Reserved(Keyword::Let),
        "mut" => Identifier::Reserved(Keyword::Mut),
        "fn" => Identifier::Reserved(Keyword::Function),
        "trait" => Identifier::Reserved(Keyword::Trait),
        "struct" => Identifier::Reserved(Keyword::Struct),
        "type" => Identifier::Reserved(Keyword::Type),
        "enum" => Identifier::Reserved(Keyword::Enum),
        "impl" => Identifier::Reserved(Keyword::Impl),
        "mod" => Identifier::Reserved(Keyword::Module),
        "Self" => Identifier::Reserved(Keyword::TypeSelf),
        "self" => Identifier::Reserved(Keyword::VariableSelf),
        "pub" => Identifier::Reserved(Keyword::Public),
        "async" => Identifier::Reserved(Keyword::Async),
        "await" => Identifier::Reserved(Keyword::Await),
        "use" => Identifier::Reserved(Keyword::Use),
        "super" => Identifier::Reserved(Keyword::Super),
        "as" => Identifier::Reserved(Keyword::As),
        "_" => Identifier::Reserved(Keyword::Placeholder),

        _ => Identifier::Unreserved(string),
    };

    Ok((s, identifier))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keyword() {
        macro_rules! keywords {
            ($($variant:ident,)+) => {
                vec![$(Token::Keyword(Keyword::$variant),)+]
            };
        }

        let source = r#"if else for while loop in break continue match return yield where
const static let mut fn trait struct type enum impl mod Self self pub async await use super as _"#;
        assert_eq!(
            lex(source).map(|(_, token_list)| token_list),
            Ok(keywords![
                If,
                Else,
                For,
                While,
                Loop,
                In,
                Break,
                Continue,
                Match,
                Return,
                Yield,
                Where,
                Const,
                Static,
                Let,
                Mut,
                Function,
                Trait,
                Struct,
                Type,
                Enum,
                Impl,
                Module,
                TypeSelf,
                VariableSelf,
                Public,
                Async,
                Await,
                Use,
                Super,
                As,
                Placeholder,
            ])
        );
    }
}
