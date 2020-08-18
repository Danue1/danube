use super::*;

pub(super) fn parse_trait_item_function_node(s: Tokens) -> ParseResult<TraitItemFunctionNode> {
  map(
    tuple((
      parse_keyword(Keyword::Function),
      parse_ident_node,
      opt(parse_generic_node),
      parse_function_argument_list,
      opt(parse_function_type),
      parse_function_body,
    )),
    |(_, ident, generic, argument_list, return_type, body)| TraitItemFunctionNode {
      ident,
      generic,
      argument_list,
      return_type,
      body,
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

fn parse_function_body(s: Tokens) -> ParseResult<Option<Vec<StatementNode>>> {
  alt((
    map(parse_function_body_shrotcut, Some),
    map(parse_function_body_longcut, Some),
    map(parse_symbol(Symbol::Semicolon), |_| None),
  ))(s)
}

fn parse_function_body_shrotcut(s: Tokens) -> ParseResult<Vec<StatementNode>> {
  map(
    tuple((
      parse_symbol(Symbol::Assign),
      parse_expression_node,
      parse_symbol(Symbol::Semicolon),
    )),
    |(_, expression, _)| vec![StatementNode::Expression(expression)],
  )(s)
}

fn parse_function_body_longcut(s: Tokens) -> ParseResult<Vec<StatementNode>> {
  map(
    tuple((
      parse_symbol(Symbol::LeftBrace),
      many0(parse_statement_node),
      parse_symbol(Symbol::RightBrace),
    )),
    |(_, statement_list, _)| statement_list,
  )(s)
}
