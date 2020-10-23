use super::*;

pub(super) fn parse_struct(t: Tokens) -> ParseResult<Struct> {
    parse_attributed(parse_struct_node)(t)
}
