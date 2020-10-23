use super::*;

pub(super) fn parse_trait_item_constant_node(t: Tokens) -> ParseResult<TraitItemConstantNode> {
    map(
        tuple((
            parse_keyword(Keyword::Const),
            parse_ident_node,
            parse_symbol(Symbol::Colon),
            parse_type_kind,
            opt(map(
                tuple((parse_symbol(Symbol::Assign), parse_value_kind)),
                |(_, value)| value,
            )),
            parse_symbol(Symbol::Semicolon),
        )),
        |(_, ident, _, ty, default_value, _)| TraitItemConstantNode {
            ident,
            ty,
            default_value,
        },
    )(t)
}
