use super::*;

pub(super) fn parse_symbol(s: LexSpan) -> LexResult<Symbol> {
    parse_symbol0(s)
}

fn parse_symbol0(s: LexSpan) -> LexResult<Symbol> {
    let (s3, char3) = take(3usize)(s)?;
    match char3.fragment().to_owned() {
        "..=" => Ok((s3, Symbol::RangeOpen)),
        "**=" => Ok((s3, Symbol::ExpAssign)),
        "&&=" => Ok((s3, Symbol::AndAssign)),
        "||=" => Ok((s3, Symbol::OrAssign)),
        "<<=" => Ok((s3, Symbol::BitLeftAssign)),
        ">>=" => Ok((s3, Symbol::BitRightAssign)),

        _ => {
            let (s1, char2) = take(2usize)(s)?;
            match char2.fragment().to_owned() {
                "|>" => Ok((s1, Symbol::ChainArrow)),
                "->" => Ok((s1, Symbol::ReturnArrow)),
                "=>" => Ok((s1, Symbol::BranchArrow)),
                ".." => Ok((s1, Symbol::RangeClose)),
                "::" => Ok((s1, Symbol::DoubleColon)),
                "==" => Ok((s1, Symbol::Equal)),
                "!=" => Ok((s1, Symbol::NotEqual)),
                "+=" => Ok((s1, Symbol::AddAssign)),
                "-=" => Ok((s1, Symbol::SubAssign)),
                "*=" => Ok((s1, Symbol::MulAssign)),
                "/=" => Ok((s1, Symbol::DivAssign)),
                "%=" => Ok((s1, Symbol::ModAssign)),
                "**" => Ok((s1, Symbol::Exp)),
                "&&" => Ok((s1, Symbol::And)),
                "||" => Ok((s1, Symbol::Or)),
                "&=" => Ok((s1, Symbol::BitAndAssign)),
                "|=" => Ok((s1, Symbol::BitOrAssign)),
                "^=" => Ok((s1, Symbol::BitXorAssign)),
                "<<" => Ok((s1, Symbol::BitLeft)),
                ">>" => Ok((s1, Symbol::BitRight)),
                ">=" => Ok((s1, Symbol::GreaterThanOrEqual)),
                "<=" => Ok((s1, Symbol::LessThanOrEqual)),

                _ => {
                    let (s, char1) = take(1usize)(s)?;
                    match char1.fragment().to_owned() {
                        "(" => Ok((s, Symbol::LeftParens)),
                        ")" => Ok((s, Symbol::RightParens)),
                        "[" => Ok((s, Symbol::LeftBracket)),
                        "]" => Ok((s, Symbol::RightBracket)),
                        "{" => Ok((s, Symbol::LeftBrace)),
                        "}" => Ok((s, Symbol::RightBrace)),
                        "#" => Ok((s, Symbol::Hashtag)),
                        "." => Ok((s, Symbol::Dot)),
                        "," => Ok((s, Symbol::Comma)),
                        ":" => Ok((s, Symbol::Colon)),
                        ";" => Ok((s, Symbol::Semicolon)),
                        "=" => Ok((s, Symbol::Assign)),
                        "+" => Ok((s, Symbol::Add)),
                        "-" => Ok((s, Symbol::Sub)),
                        "*" => Ok((s, Symbol::Mul)),
                        "/" => Ok((s, Symbol::Div)),
                        "%" => Ok((s, Symbol::Mod)),
                        "!" => Ok((s, Symbol::Not)),
                        "?" => Ok((s, Symbol::Question)),
                        "&" => Ok((s, Symbol::BitAnd)),
                        "|" => Ok((s, Symbol::BitOr)),
                        "~" => Ok((s, Symbol::BitNot)),
                        "^" => Ok((s, Symbol::BitXor)),
                        ">" => Ok((s, Symbol::GreaterThan)),
                        "<" => Ok((s, Symbol::LessThan)),

                        _ => Err(nom::Err::Error(nom::error_position!(
                            s,
                            nom::error::ErrorKind::Count
                        ))),
                    }
                }
            }
        }
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
