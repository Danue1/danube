use super::*;

pub(super) fn parse_condition_node(s: Tokens) -> ParseResult<ConditionNode> {
  map(
    tuple((
      opt(map(
        tuple((
          parse_keyword(Keyword::Let),
          parse_immutablity_kind,
          parse_pattern_kind,
          parse_symbol(Symbol::Assign),
        )),
        |(_, immutablity, pattern, _)| (immutablity, pattern),
      )),
      parse_expression_kind,
    )),
    |(pattern, value)| ConditionNode {
      pattern,
      value: Box::new(value),
    },
  )(s)
}
