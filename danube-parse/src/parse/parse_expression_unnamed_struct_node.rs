use super::*;

pub(super) fn parse_expression_unnamed_struct_node(
    t: Tokens,
) -> ParseResult<ExpressionUnnamedStructNode> {
    map(
        tuple((
            opt(parse_path_node),
            parse_symbol(Symbol::LeftParens),
            separated_nonempty_list(parse_symbol(Symbol::Comma), parse_pattern_kind),
            parse_symbol(Symbol::RightParens),
        )),
        |(path, _, field_list, _)| ExpressionUnnamedStructNode { path, field_list },
    )(t)
}
