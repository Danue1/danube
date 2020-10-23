use super::*;

pub(super) fn parse_ident_node(t: Tokens) -> ParseResult<IdentNode> {
    map(parse_identifier, |identifier| IdentNode { raw: identifier })(t)
}
