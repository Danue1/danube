use super::*;

pub(super) fn parse_closure_node(s: Tokens) -> ParseResult<ClosureNode> {
  let (s, argument_list) = alt((
    map(parse_symbol(Symbol::Or), |_| vec![]),
    map(
      tuple((
        parse_symbol(Symbol::BitOr),
        separated_list(parse_symbol(Symbol::Comma), parse_closure_argument_node),
        parse_symbol(Symbol::BitOr),
      )),
      |(_, argument_list, _)| argument_list,
    ),
  ))(s)?;
  let (s, return_type) = opt(preceded(parse_symbol(Symbol::ReturnArrow), parse_type_kind))(s)?;
  let (s, block) = if return_type.is_some() {
    parse_block_node(s)?
  } else {
    alt((
      map(parse_expression_kind, |expression| BlockNode {
        statement_list: vec![StatementKind::ExpressionKind(expression)],
      }),
      parse_block_node,
    ))(s)?
  };
  let node = ClosureNode {
    argument_list,
    return_type,
    block,
  };

  Ok((s, node))
}
