use super::*;

pub(super) fn parse_enum_unnamed_variant_node(t: Tokens) -> ParseResult<EnumUnnamedVariantNode> {
    map(
        tuple((
            parse_symbol(Symbol::LeftParens),
            separated_nonempty_list(parse_symbol(Symbol::Comma), parse_type_kind),
            opt(parse_symbol(Symbol::Comma)),
            parse_symbol(Symbol::RightParens),
        )),
        |(_, node_list, _, _)| EnumUnnamedVariantNode { node_list },
    )(t)
}
