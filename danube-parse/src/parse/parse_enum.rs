use super::*;

pub(super) fn parse_enum(t: Tokens) -> ParseResult<Enum> {
    parse_attributed(parse_enum_node)(t)
}
