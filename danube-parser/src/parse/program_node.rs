use super::*;

pub(super) fn parse_program_node(s: Tokens) -> ParseResult<ProgramNode> {
  map(
    all_consuming(tuple((many0(parse_feature_node), parse_module_node))),
    |(feature_list, module)| ProgramNode {
      feature_list,
      module,
    },
  )(s)
}
