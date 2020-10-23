use super::*;

pub(super) fn parse_enum_node(t: Tokens) -> ParseResult<EnumNode> {
    map(
        tuple((
            parse_visibility_kind,
            parse_keyword(Keyword::Enum),
            parse_ident_node,
            parse_generic_node_list,
            parse_symbol(Symbol::LeftBrace),
            separated_nonempty_list(
                parse_symbol(Symbol::Comma),
                tuple((parse_ident_node, parse_enum_variant_kind)),
            ),
            opt(parse_symbol(Symbol::Comma)),
            parse_symbol(Symbol::LeftBrace),
        )),
        |(visibility, _, ident, generic_list, _, variant_list, _, _)| EnumNode {
            visibility,
            ident,
            generic_list,
            variant_list,
        },
    )(t)
}
