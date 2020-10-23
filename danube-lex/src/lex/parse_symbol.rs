use super::*;

pub(super) fn parse_symbol(s: LexSpan) -> LexResult<Symbol> {
    alt((parse_symbol3, parse_symbol2, parse_symbol1))(s)
}

fn parse_symbol3(s: LexSpan) -> LexResult<Symbol> {
    let (ss, c) = take(3usize)(s)?;
    match c.fragment().to_owned() {
        "..=" => Ok((ss, Symbol::RangeOpen)),
        "**=" => Ok((ss, Symbol::ExpAssign)),
        "&&=" => Ok((ss, Symbol::AndAssign)),
        "||=" => Ok((ss, Symbol::OrAssign)),
        "<<=" => Ok((ss, Symbol::BitLeftAssign)),
        ">>=" => Ok((ss, Symbol::BitRightAssign)),

        _ => Err(nom::Err::Error(nom::error_position!(
            s,
            nom::error::ErrorKind::Count
        ))),
    }
}

fn parse_symbol2(s: LexSpan) -> LexResult<Symbol> {
    let (ss, c) = take(2usize)(s)?;
    match c.fragment().to_owned() {
        "|>" => Ok((ss, Symbol::ChainArrow)),
        "->" => Ok((ss, Symbol::ReturnArrow)),
        "=>" => Ok((ss, Symbol::BranchArrow)),
        ".." => Ok((ss, Symbol::RangeClose)),
        "::" => Ok((ss, Symbol::DoubleColon)),
        "==" => Ok((ss, Symbol::Equal)),
        "!=" => Ok((ss, Symbol::NotEqual)),
        "+=" => Ok((ss, Symbol::AddAssign)),
        "-=" => Ok((ss, Symbol::SubAssign)),
        "*=" => Ok((ss, Symbol::MulAssign)),
        "/=" => Ok((ss, Symbol::DivAssign)),
        "%=" => Ok((ss, Symbol::ModAssign)),
        "**" => Ok((ss, Symbol::Exp)),
        "&&" => Ok((ss, Symbol::And)),
        "||" => Ok((ss, Symbol::Or)),
        "&=" => Ok((ss, Symbol::BitAndAssign)),
        "|=" => Ok((ss, Symbol::BitOrAssign)),
        "^=" => Ok((ss, Symbol::BitXorAssign)),
        "<<" => Ok((ss, Symbol::BitLeft)),
        ">>" => Ok((ss, Symbol::BitRight)),
        ">=" => Ok((ss, Symbol::GreaterThanOrEqual)),
        "<=" => Ok((ss, Symbol::LessThanOrEqual)),

        _ => Err(nom::Err::Error(nom::error_position!(
            s,
            nom::error::ErrorKind::Count
        ))),
    }
}

fn parse_symbol1(s: LexSpan) -> LexResult<Symbol> {
    let (ss, c) = take(1usize)(s)?;
    match c.fragment().to_owned() {
        "(" => Ok((ss, Symbol::LeftParens)),
        ")" => Ok((ss, Symbol::RightParens)),
        "[" => Ok((ss, Symbol::LeftBracket)),
        "]" => Ok((ss, Symbol::RightBracket)),
        "{" => Ok((ss, Symbol::LeftBrace)),
        "}" => Ok((ss, Symbol::RightBrace)),
        "#" => Ok((ss, Symbol::Hashtag)),
        "." => Ok((ss, Symbol::Dot)),
        "," => Ok((ss, Symbol::Comma)),
        ":" => Ok((ss, Symbol::Colon)),
        ";" => Ok((ss, Symbol::Semicolon)),
        "=" => Ok((ss, Symbol::Assign)),
        "+" => Ok((ss, Symbol::Add)),
        "-" => Ok((ss, Symbol::Sub)),
        "*" => Ok((ss, Symbol::Mul)),
        "/" => Ok((ss, Symbol::Div)),
        "%" => Ok((ss, Symbol::Mod)),
        "!" => Ok((ss, Symbol::Not)),
        "?" => Ok((ss, Symbol::Question)),
        "&" => Ok((ss, Symbol::BitAnd)),
        "|" => Ok((ss, Symbol::BitOr)),
        "~" => Ok((ss, Symbol::BitNot)),
        "^" => Ok((ss, Symbol::BitXor)),
        ">" => Ok((ss, Symbol::GreaterThan)),
        "<" => Ok((ss, Symbol::LessThan)),

        _ => Err(nom::Err::Error(nom::error_position!(
            s,
            nom::error::ErrorKind::Count
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn symbol() {
        macro_rules! tokens {
        ($($variant:ident,)+) => {
            vec![$(Token::Symbol(Symbol::$variant),)+]
        };
    }

        let source = "
    ( ) [ ] { }
    # |> -> =>
    ..= .. . , :: : ; == = !=
    += -= **= *= /= %= &&= ||= &= |= ^= <<= >>=
    + - ** * / % && || ! ?
    & | ~ ^
    << >> >= <= > <
    ";
        assert_eq!(
            lex(source).map(|(_, token_list)| token_list),
            Ok(tokens![
                LeftParens,
                RightParens,
                LeftBracket,
                RightBracket,
                LeftBrace,
                RightBrace,
                Hashtag,
                ChainArrow,
                ReturnArrow,
                BranchArrow,
                RangeOpen,
                RangeClose,
                Dot,
                Comma,
                DoubleColon,
                Colon,
                Semicolon,
                Equal,
                Assign,
                NotEqual,
                AddAssign,
                SubAssign,
                ExpAssign,
                MulAssign,
                DivAssign,
                ModAssign,
                AndAssign,
                OrAssign,
                BitAndAssign,
                BitOrAssign,
                BitXorAssign,
                BitLeftAssign,
                BitRightAssign,
                Add,
                Sub,
                Exp,
                Mul,
                Div,
                Mod,
                And,
                Or,
                Not,
                Question,
                BitAnd,
                BitOr,
                BitNot,
                BitXor,
                BitLeft,
                BitRight,
                GreaterThanOrEqual,
                LessThanOrEqual,
                GreaterThan,
                LessThan,
            ])
        );
    }
}
