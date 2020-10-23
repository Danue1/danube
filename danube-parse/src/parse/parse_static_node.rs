use super::*;

pub(super) fn parse_static_node(t: Tokens) -> ParseResult<StaticNode> {
    map(
        tuple((
            parse_visibility_kind,
            parse_keyword(Keyword::Static),
            parse_ident_node,
            parse_symbol(Symbol::Colon),
            parse_type_kind,
            parse_symbol(Symbol::Assign),
            parse_value_kind,
            parse_symbol(Symbol::Semicolon),
        )),
        |(visibility, _, ident, _, ty, _, value, _)| StaticNode {
            visibility,
            ident,
            ty,
            value,
        },
    )(t)
}
