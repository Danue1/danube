use super::*;

pub fn parse_expression_kind(t: Tokens) -> ParseResult<ExpressionKind> {
    map(parse_literal_kind, ExpressionKind::Literal)(t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn int() {
        let source = r#"1"#;
        assert_eq!(
            parse_by(source, parse_expression_kind),
            ExpressionKind::Literal(LiteralKind::Int(1))
        );
    }

    #[test]
    fn float() {
        let source = r#"1.0"#;
        assert_eq!(
            parse_by(source, parse_expression_kind),
            ExpressionKind::Literal(LiteralKind::Float(1.0))
        );
    }

    #[test]
    fn string() {
        let source = r#""a""#;
        assert_eq!(
            parse_by(source, parse_expression_kind),
            ExpressionKind::Literal(LiteralKind::String("a".to_owned()))
        );
    }
}
