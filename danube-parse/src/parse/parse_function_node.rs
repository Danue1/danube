use super::*;

pub(super) fn parse_function_node(t: Tokens) -> ParseResult<FunctionNode> {
    map(
        tuple((
            parse_visibility_kind,
            parse_keyword(Keyword::Function),
            parse_ident_node,
            parse_generic_node_list,
            parse_function_argument_node_list,
            parse_return_type,
            parse_function_body,
        )),
        |(visibility, _, ident, generic_list, (self_type, argument_list), return_type, block)| {
            FunctionNode {
                visibility,
                ident,
                generic_list,
                self_type,
                argument_list,
                return_type,
                block,
            }
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
        |(_, expression, _)| BlockNode {
            statement_list: vec![StatementKind::Expression(expression)],
        },
    )(t)
}
