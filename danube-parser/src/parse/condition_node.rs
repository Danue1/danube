use super::*;

pub(super) fn parse_condition_node(s: Tokens) -> ParseResult<ConditionNode> {
  map(
    tuple((
      opt(map(
        tuple((
          parse_keyword(Keyword::Let),
          parse_immutablity,
          parse_pattern_node,
          parse_symbol(Symbol::Assign),
        )),
        |(_, immutablity, pattern, _)| (immutablity, pattern),
      )),
      parse_expression_node,
    )),
    |(pattern, value)| ConditionNode {
      pattern,
      value: Box::new(value),
    },
  )(s)
}
