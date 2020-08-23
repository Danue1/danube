use super::*;

pub(super) fn parse_block_node(s: Tokens) -> ParseResult<BlockNode> {
  map(
    tuple((
      parse_symbol(Symbol::LeftBrace),
      many0(parse_statement_node),
      parse_symbol(Symbol::RightBrace),
    )),
    |(_, statement_list, _)| BlockNode { statement_list },
  )(s)
}
