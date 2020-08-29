use super::*;

pub(super) fn parse_use_root_node(s: Tokens) -> ParseResult<UseRootNode> {
  map(
    tuple((
      parse_use_root_ident_kind,
      parse_symbol(Symbol::DoubleColon),
      parse_use_kind(parse_use_extra_kind),
    )),
    |(ident, _, extra)| UseRootNode { ident, extra },
  )(s)
}
