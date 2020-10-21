use super::*;

#[derive(Debug, PartialEq)]
pub(super) enum Numeric {
    Int(i64),
    Float(f64),
}

pub(super) fn parse_numeric(s: LexSpan) -> LexResult<Numeric> {
    let is_digit = |c| matches!(c, '0'..='9');
    let is_nonzero_digit = |c| matches!(c, '1'..='9');
    let sign_to_int = |sign: Option<_>| if sign.is_some() { 1 } else { 0 };
    let len = |s: LexSpan| s.fragment().len();

    let (ss, majority) = alt((
        map(tag("0"), |_| 1),
        map(
            tuple((take_while1(is_nonzero_digit), take_while(is_digit))),
            |(nonzero_digit, digit)| len(nonzero_digit) + len(digit),
        ),
    ))(s)?;

    let (ss, minority) = opt(map(
        tuple((char('.'), take_while(is_digit))),
        |(_, minority): (_, LexSpan)| 1 + len(minority),
    ))(ss)?;

    let (_, exponential) = opt(map(
        tuple((
            alt((char('e'), char('E'))),
            opt(alt((char('+'), char('-')))),
            take_while1(is_digit),
        )),
        |(_, sign, digit)| 1 + sign_to_int(sign) + len(digit),
    ))(ss)?;

    let numeric = match (minority, exponential) {
        (Some(n), Some(m)) => Some(n + m),
        (Some(n), _) => Some(n),
        (_, Some(n)) => Some(n),
        _ => None,
    };

    match numeric {
        Some(n) => {
            let (s, numeric) = take(majority + n)(s)?;
            Ok((s, Numeric::Float(numeric.fragment().parse().unwrap())))
        }
        None => {
            let (s, numeric) = take(majority)(s)?;
            Ok((s, Numeric::Int(numeric.fragment().parse().unwrap())))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn int() {
        let source = r#"1"#;
        assert_eq!(
            lex(source).map(|(_, token_list)| token_list),
            Ok(vec![Token::IntLiteral(1)])
        );
    }

    #[test]
    fn float() {
        let source = r#"1.0"#;
        assert_eq!(
            lex(source).map(|(_, token_list)| token_list),
            Ok(vec![Token::FloatLiteral(1.0)])
        );

        let source = r#"-1.0"#;
        assert_eq!(
            lex(source).map(|(_, token_list)| token_list),
            Ok(vec![Token::Symbol(Symbol::Sub), Token::FloatLiteral(1.0)])
        );

        let source = r#"1e0"#;
        assert_eq!(
            lex(source).map(|(_, token_list)| token_list),
            Ok(vec![Token::FloatLiteral(1e0)])
        );

        let source = r#"-1e0"#;
        assert_eq!(
            lex(source).map(|(_, token_list)| token_list),
            Ok(vec![Token::Symbol(Symbol::Sub), Token::FloatLiteral(1e0)])
        );

        let source = r#"1.0e0"#;
        assert_eq!(
            lex(source).map(|(_, token_list)| token_list),
            Ok(vec![Token::FloatLiteral(1.0e0)])
        );

        let source = r#"-1.0e0"#;
        assert_eq!(
            lex(source).map(|(_, token_list)| token_list),
            Ok(vec![Token::Symbol(Symbol::Sub), Token::FloatLiteral(1.0e0)])
        );
    }
}
