use super::*;

pub(super) fn parse_type_array_node(s: Tokens) -> ParseResult<TypeArrayNode> {
  map(
    tuple((
      parse_symbol(Symbol::LeftBracket),
      parse_type_node,
      parse_symbol(Symbol::Semicolon),
      parse_int,
      parse_symbol(Symbol::RightBracket),
    )),
    |(_, ty, _, size, _)| TypeArrayNode {
      ty,
      size: size as usize,
    },
  )(s)
}