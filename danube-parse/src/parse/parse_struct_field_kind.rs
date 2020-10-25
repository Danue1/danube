use super::*;

pub(super) fn parse_struct_field_kind(t: Tokens) -> ParseResult<StructFieldKind> {
    alt((
        map(parse_struct_unnamed_field_node, StructFieldKind::Unnamed),
        map(parse_struct_named_field_node, StructFieldKind::Named),
    ))(t)
}

fn parse_struct_unnamed_field_node(t: Tokens) -> ParseResult<Vec<(VisibilityKind, TypeKind)>> {
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
        |(_, node_list, _, _)| node_list,
    )(t)
}

fn parse_struct_named_field_node(
    t: Tokens,
) -> ParseResult<Vec<(VisibilityKind, IdentNode, TypeKind)>> {
    map(
        tuple((
            parse_symbol(Symbol::LeftBrace),
            separated_nonempty_list(
                parse_symbol(Symbol::Colon),
                tuple((
                    parse_visibility_kind,
                    parse_ident_node,
                    preceded(parse_symbol(Symbol::Colon), parse_type_kind),
                )),
            ),
            opt(parse_symbol(Symbol::Comma)),
            parse_symbol(Symbol::RightBrace),
        )),
        |(_, node_list, _, _)| node_list,
    )(t)
}
