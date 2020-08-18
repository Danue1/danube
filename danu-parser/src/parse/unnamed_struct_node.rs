use super::*;

pub(super) fn parse_unnamed_struct_node(s: Tokens) -> ParseResult<UnnamedStructNode> {
  map(
    tuple((
      parse_path_node,
      parse_symbol(Symbol::LeftParens),
      separated_nonempty_list(parse_symbol(Symbol::Comma), parse_pattern_node),
      parse_symbol(Symbol::RightParens),
    )),
    |(path, _, field_list, _)| UnnamedStructNode { path, field_list },
  )(s)
}
