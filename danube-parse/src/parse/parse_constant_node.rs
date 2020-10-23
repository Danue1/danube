use super::*;

pub(super) fn parse_constant_node(t: Tokens) -> ParseResult<ConstantNode> {
    map(
        tuple((
            parse_visibility_kind,
            parse_keyword(Keyword::Const),
            parse_ident_node,
            parse_symbol(Symbol::Colon),
            parse_type_kind,
            parse_symbol(Symbol::Assign),
            parse_value_kind,
            parse_symbol(Symbol::Semicolon),
        )),
        |(visibility, _, ident, _, ty, _, value, _)| ConstantNode {
            visibility,
            ident,
            ty,
            value,
        },
    )(t)
}
