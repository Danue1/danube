use super::*;

pub(super) fn parse_module(t: Tokens) -> ParseResult<Module> {
    parse_attributed(parse_module_node)(t)
}
