use super::*;

pub(super) fn parse_struct_unnamed_field_node(t: Tokens) -> ParseResult<StructUnnamedFieldNode> {
    map(
        tuple((
            parse_symbol(Symbol::LeftParens),
            separated_nonempty_list(
                parse_symbol(Symbol::Comma),
                tuple((parse_visibility_kind, parse_type_kind)),
            ),
            opt(parse_symbol(Symbol::Comma)),
            parse_symbol(Symbol::RightParens),
        )),
        |(_, node_list, _, _)| StructUnnamedFieldNode { node_list },
    )(t)
}
