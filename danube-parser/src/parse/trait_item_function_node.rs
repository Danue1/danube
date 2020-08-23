use super::*;

pub(super) fn parse_trait_item_function_node(s: Tokens) -> ParseResult<TraitItemFunctionNode> {
  map(
    tuple((
      opt(parse_keyword(Keyword::Async)),
      parse_keyword(Keyword::Function),
      parse_ident_node,
      opt(parse_generic_node),
      parse_function_argument_list,
      opt(parse_function_type),
      parse_function_body,
    )),
    |(is_async, _, ident, generic, argument_list, return_type, block)| TraitItemFunctionNode {
      is_async: is_async.is_some(),
      ident,
      generic,
      argument_list,
      return_type,
      block,
    },
  )(s)
}

fn parse_function_argument_list(s: Tokens) -> ParseResult<Vec<FunctionArgumentNode>> {
  map(
    tuple((
      parse_symbol(Symbol::LeftParens),
      separated_list(parse_symbol(Symbol::Comma), parse_function_argument_node),
      opt(parse_symbol(Symbol::Comma)),
      parse_symbol(Symbol::RightParens),
    )),
    |(_, argument_list, _, _)| argument_list,
  )(s)
}

fn parse_function_type(s: Tokens) -> ParseResult<TypeNode> {
  map(
    tuple((parse_symbol(Symbol::ReturnArrow), parse_type_node)),
    |(_, ty)| ty,
  )(s)
}

fn parse_function_body(s: Tokens) -> ParseResult<Option<BlockNode>> {
  alt((
    map(parse_function_body_shrotcut, Some),
    map(parse_block_node, Some),
    map(parse_symbol(Symbol::Semicolon), |_| None),
  ))(s)
}

fn parse_function_body_shrotcut(s: Tokens) -> ParseResult<BlockNode> {
  map(
    tuple((
      parse_symbol(Symbol::Assign),
      parse_expression_node,
      parse_symbol(Symbol::Semicolon),
    )),
    |(_, expression, _)| BlockNode {
      statement_list: vec![StatementNode::Expression(expression)],
    },
  )(s)
}
