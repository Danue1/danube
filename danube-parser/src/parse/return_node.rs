use super::*;

pub(super) fn parse_return_node(s: Tokens) -> ParseResult<ReturnNode> {
  map(
    tuple((
      parse_keyword(Keyword::Return),
      opt(parse_expression_node),
      opt(parse_symbol(Symbol::Semicolon)),
    )),
    |(_, expression, _)| ReturnNode {
      value: expression.map(Box::new),
    },
  )(s)
}

#[cfg(test)]
mod tests {
  use super::*;

  fn compile(s: &str) -> ReturnNode {
    let (_, token_list) = lex(s).unwrap();
    let (_, node) = parse_return_node(Tokens::new(&token_list)).unwrap();

    node
  }

  #[test]
  fn return_unit() {
    let source = "return";
    assert_eq!(compile(source), ReturnNode { value: None });
  }

  #[test]
  fn return_unit_semicolon() {
    let source = "return;";
    assert_eq!(compile(source), ReturnNode { value: None });
  }

  #[test]
  fn return_value() {
    let source = "return true";
    assert_eq!(
      compile(source),
      ReturnNode {
        value: Some(Box::new(ExpressionNode::Literal(LiteralValueNode::Bool(
          true
        ))))
      }
    );
  }

  #[test]
  fn return_value_semicolon() {
    let source = "return true;";
    assert_eq!(
      compile(source),
      ReturnNode {
        value: Some(Box::new(ExpressionNode::Literal(LiteralValueNode::Bool(
          true
        ))))
      }
    );
  }
}
