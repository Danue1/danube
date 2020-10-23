use super::*;

pub(super) fn parse_function(t: Tokens) -> ParseResult<Function> {
    parse_attributed(parse_function_node)(t)
}
