use super::*;

pub(super) fn parse_program_node(s: Tokens) -> ParseResult<ProgramNode> {
  map(
    all_consuming(tuple((many0(parse_feature_node), many0(parse_item_node)))),
    |(feature_list, item_list)| ProgramNode {
      feature_list,
      item_list,
    },
  )(s)
}
