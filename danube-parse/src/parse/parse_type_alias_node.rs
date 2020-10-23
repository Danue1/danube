use super::*;

pub(super) fn parse_type_alias_node(t: Tokens) -> ParseResult<TypeAliasNode> {
    map(
        tuple((
            parse_visibility_kind,
            parse_keyword(Keyword::Type),
            parse_ident_node,
            parse_symbol(Symbol::Assign),
            parse_type_kind,
            parse_symbol(Symbol::Semicolon),
        )),
        |(visibility, _, ident, _, ty, _)| TypeAliasNode {
            visibility,
            ident,
            ty,
        },
    )(t)
}
