use super::*;

pub(super) fn parse_use_root_node(s: Tokens) -> ParseResult<UseRootNode> {
  map(
    tuple((
      parse_use_root_ident,
      parse_symbol(Symbol::DoubleColon),
      parse_use_kind(parse_use_extra),
    )),
    |(ident, _, extra)| UseRootNode { ident, extra },
  )(s)
}
