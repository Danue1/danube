use super::*;

pub(super) fn parse_implement_item_kind(s: Tokens) -> ParseResult<ImplementItemKind> {
  let (s, attribute_list) = many0(parse_attribute_node)(s)?;

  match alt((
    map(parse_implement_output_type_node, Kind::OutputType),
    map(parse_constant_node, Kind::Constant),
    map(parse_function_node, Kind::Function),
  ))(s)?
  {
    (s, Kind::OutputType(node)) => Ok((s, ImplementItemKind::OutputType(attribute_list, node))),
    (s, Kind::Constant(node)) => Ok((s, ImplementItemKind::Constant(attribute_list, node))),
    (s, Kind::Function(node)) => Ok((s, ImplementItemKind::Function(attribute_list, node))),
  }
}

enum Kind {
  OutputType(ImplementOutputTypeNode),
  Constant(ConstantNode),
  Function(FunctionNode),
}
