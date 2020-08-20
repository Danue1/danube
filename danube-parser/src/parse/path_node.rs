use super::*;

pub(super) fn parse_path_node(s: Tokens) -> ParseResult<PathNode> {
  map(
    separated_nonempty_list(parse_symbol(Symbol::DoubleColon), parse_ident_node),
    |ident_list| PathNode { ident_list },
  )(s)
}
