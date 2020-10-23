use super::*;

pub(super) fn parse_struct_named_field_node(t: Tokens) -> ParseResult<StructNamedFieldNode> {
    map(
        tuple((
            parse_symbol(Symbol::LeftBrace),
            separated_nonempty_list(
                parse_symbol(Symbol::Colon),
                tuple((
                    parse_immutablity_kind,
                    parse_ident_node,
                    preceded(parse_symbol(Symbol::Colon), parse_type_kind),
                )),
            ),
            opt(parse_symbol(Symbol::Comma)),
            parse_symbol(Symbol::RightBrace),
        )),
        |(_, node_list, _, _)| StructNamedFieldNode { node_list },
    )(t)
}
