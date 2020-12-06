use super::*;

pub fn parse_ident_node(t: Tokens) -> ParseResult<IdentNode> {
    map(parse_identifier, |identifier| ident!(identifier))(t)
}
