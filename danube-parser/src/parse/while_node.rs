use super::*;

pub(super) fn parse_while_node(s: Tokens) -> ParseResult<WhileNode> {
  map(
    tuple((
      parse_keyword(Keyword::While),
      parse_expression_node,
      parse_symbol(Symbol::LeftBrace),
      many0(parse_statement_node),
      parse_symbol(Symbol::RightBrace),
    )),
    |(_, condition, _, body, _)| WhileNode {
      condition: Box::new(condition),
      body,
    },
  )(s)
}

#[cfg(test)]
mod tests {
  use super::*;

  fn compile(s: &str) -> WhileNode {
    let (_, token_list) = lex(s).unwrap();
    match parse_while_node(Tokens::new(&token_list)) {
      Ok((_, node)) => node,
      Err(error) => {
        dbg!(error);
        panic!()
      }
    }
  }

  #[test]
  fn test() {
    let source = "while true { }";
    assert_eq!(
      compile(source),
      WhileNode {
        condition: Box::new(ExpressionNode::Literal(LiteralValueNode::Bool(true)),),
        body: vec![]
      }
    );
  }
}
