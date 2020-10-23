use super::*;

pub(super) fn parse_trait_item_function_node(t: Tokens) -> ParseResult<TraitItemFunctionNode> {
    map(
        tuple((
            parse_keyword(Keyword::Function),
            parse_ident_node,
            parse_generic_node_list,
            parse_function_argument_node_list,
            opt(parse_function_type),
            parse_function_body,
        )),
        |(_, ident, generic_list, (self_type, argument_list), return_type, block)| {
            TraitItemFunctionNode {
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

fn parse_function_type(t: Tokens) -> ParseResult<TypeKind> {
    map(
        tuple((parse_symbol(Symbol::ReturnArrow), parse_type_kind)),
        |(_, ty)| ty,
    )(t)
}

fn parse_function_body(t: Tokens) -> ParseResult<Option<BlockNode>> {
    alt((
        map(parse_function_body_shrotcut, Some),
        map(parse_block_node, Some),
        map(parse_symbol(Symbol::Semicolon), |_| None),
    ))(t)
}

fn parse_function_body_shrotcut(t: Tokens) -> ParseResult<BlockNode> {
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
