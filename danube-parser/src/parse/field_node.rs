use super::*;

pub(super) fn parse_field_node(s: Tokens) -> ParseResult<FieldNode> {
  map(
    tuple((
      parse_ident_node,
      opt(map(
        tuple((parse_symbol(Symbol::Colon), parse_pattern_kind)),
        |(_, pattern)| pattern,
      )),
    )),
    |(ident, pattern)| FieldNode { ident, pattern },
  )(s)
}
