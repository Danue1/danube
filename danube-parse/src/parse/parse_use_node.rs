use super::*;

pub(super) fn parse_use_node(t: Tokens) -> ParseResult<UseNode> {
    map(
        tuple((
            parse_visibility_kind,
            parse_keyword(Keyword::Use),
            parse_use_kind(parse_use_root_node),
            parse_symbol(Symbol::Semicolon),
        )),
        |(visibility, _, kind, _)| UseNode { visibility, kind },
    )(t)
}
