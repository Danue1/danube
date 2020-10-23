use super::*;

pub(super) fn parse_expression_named_struct_node(
    t: Tokens,
) -> ParseResult<ExpressionNamedStructNode> {
    map(
        tuple((
            opt(parse_path_node),
            parse_symbol(Symbol::LeftBrace),
            separated_nonempty_list(parse_symbol(Symbol::Comma), parse_field_node),
            parse_symbol(Symbol::RightBrace),
        )),
        |(path, _, field_list, _)| ExpressionNamedStructNode { path, field_list },
    )(t)
}

fn parse_field_node(t: Tokens) -> ParseResult<(IdentNode, Option<PatternKind>)> {
    tuple((
        parse_ident_node,
        opt(map(
            tuple((parse_symbol(Symbol::Colon), parse_pattern_kind)),
            |(_, pattern)| pattern,
        )),
    ))(t)
}
