use super::*;

pub fn parse_function_node(t: Tokens) -> ParseResult<FunctionNode> {
    map(
        tuple((
            parse_keyword(Keyword::Function),
            parse_ident_node,
            parse_function_parameter_node_list,
            opt(parse_return_type),
            parse_function_body,
        )),
        |(_, ident, parameter_list, return_type, block)| FunctionNode {
            ident,
            parameter_list,
            return_type,
            block,
        },
    )(t)
}

fn parse_return_type(t: Tokens) -> ParseResult<TypeKind> {
    preceded(parse_symbol(Symbol::Colon), parse_type_kind)(t)
}

fn parse_function_body(t: Tokens) -> ParseResult<BlockNode> {
    alt((parse_function_body_shortcut, parse_block_node))(t)
}

fn parse_function_body_shortcut(t: Tokens) -> ParseResult<BlockNode> {
    map(
        tuple((
            parse_symbol(Symbol::Assign),
            parse_expression_kind,
            parse_symbol(Symbol::Semicolon),
        )),
        |(_, expression, _)| block![StatementKind::Expression(expression)],
    )(t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let source = r#"fn a() { }"#;
        assert_eq!(
            parse_by(source, parse_function_node),
            FunctionNode {
                ident: ident!("a".into()),
                parameter_list: Default::default(),
                return_type: None,
                block: block![]
            }
        );
    }
}
