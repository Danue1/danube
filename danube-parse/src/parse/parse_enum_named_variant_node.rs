use super::*;

pub(super) fn parse_enum_named_variant_node(t: Tokens) -> ParseResult<EnumNamedVariantNode> {
    map(
        tuple((
            parse_symbol(Symbol::LeftParens),
            separated_nonempty_list(
                parse_symbol(Symbol::Comma),
                tuple((
                    parse_ident_node,
                    preceded(parse_symbol(Symbol::Colon), parse_type_kind),
                )),
            ),
            opt(parse_symbol(Symbol::Comma)),
            parse_symbol(Symbol::RightParens),
        )),
        |(_, node_list, _, _)| EnumNamedVariantNode { node_list },
    )(t)
}
