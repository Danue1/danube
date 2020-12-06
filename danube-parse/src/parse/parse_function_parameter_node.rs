use super::*;

pub fn parse_function_parameter_node(t: Tokens) -> ParseResult<FunctionParametertNode> {
    map(
        tuple((
            parse_ident_node,
            opt(preceded(parse_keyword(Keyword::As), parse_ident_node)),
            opt(preceded(parse_symbol(Symbol::Colon), parse_type_kind)),
        )),
        |(argument_label, parameter_label, ty)| FunctionParametertNode {
            label: parameter_label.unwrap_or_else(|| argument_label.clone()),
            argument_label,
            ty,
        },
    )(t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let source = r#"a: int"#;
        assert_eq!(
            parse_by(source, parse_function_parameter_node),
            FunctionParametertNode {
                label: ident!("a".into()),
                argument_label: ident!("a".into()),
                ty: Some(TypeKind::Path(path![ident!("int".into())])),
            }
        );
    }

    #[test]
    fn argument_label() {
        let source = r#"a as b: int"#;
        assert_eq!(
            parse_by(source, parse_function_parameter_node),
            FunctionParametertNode {
                label: ident!("b".into()),
                argument_label: ident!("a".into()),
                ty: Some(TypeKind::Path(path![ident!("int".into())])),
            }
        );
    }
}
