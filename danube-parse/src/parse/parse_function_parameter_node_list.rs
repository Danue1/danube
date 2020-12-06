use super::*;

pub fn parse_function_parameter_node_list(t: Tokens) -> ParseResult<Vec<FunctionParametertNode>> {
    map(
        tuple((
            parse_symbol(Symbol::LeftParens),
            separated_list(parse_symbol(Symbol::Comma), parse_function_parameter_node),
            opt(parse_symbol(Symbol::Comma)),
            parse_symbol(Symbol::RightParens),
        )),
        |(_, argument_list, _, _)| argument_list,
    )(t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let source = r#"()"#;
        assert_eq!(parse_by(source, parse_function_parameter_node_list), vec![]);
    }

    #[test]
    fn a_argument() {
        let source = r#"(a: int)"#;
        assert_eq!(
            parse_by(source, parse_function_parameter_node_list),
            vec![FunctionParametertNode {
                label: ident!("a".into()),
                argument_label: ident!("a".into()),
                ty: Some(TypeKind::Path(path![ident!("int".into())]))
            }]
        );
    }

    #[test]
    fn a_argument_with_trailing_comma() {
        let source = r#"(a: int,)"#;
        assert_eq!(
            parse_by(source, parse_function_parameter_node_list),
            vec![FunctionParametertNode {
                label: ident!("a".into()),
                argument_label: ident!("a".into()),
                ty: Some(TypeKind::Path(path![ident!("int".into())]))
            }]
        );
    }
}
