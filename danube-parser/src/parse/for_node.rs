use super::*;

pub(super) fn parse_for_node(s: Tokens) -> ParseResult<ForNode> {
  map(
    tuple((
      parse_keyword(Keyword::For),
      parse_immutablity,
      parse_pattern_node,
      parse_keyword(Keyword::In),
      parse_expression_node,
      parse_symbol(Symbol::LeftBrace),
      many0(parse_statement_node),
      parse_symbol(Symbol::RightBrace),
    )),
    |(_, immutablity, pattern, _, iteration, _, body, _)| ForNode {
      immutablity,
      pattern,
      iteration: Box::new(iteration),
      body,
    },
  )(s)
}

#[cfg(test)]
mod tests {
  use super::*;

  fn compile(s: &str) -> ForNode {
    let (_, token_list) = lex(s).unwrap();
    match parse_for_node(Tokens::new(&token_list)) {
      Ok((_, node)) => node,
      Err(error) => {
        dbg!(error);
        panic!()
      }
    }
  }

  #[test]
  fn no_argument() {
    let source = "for foo in [1, 2, 3] { }";
    assert_eq!(
      compile(source),
      ForNode {
        immutablity: Immutablity::Yes,
        pattern: PatternNode::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "foo".to_owned()
          }]
        }),
        iteration: Box::new(ExpressionNode::Array(vec![
          ExpressionNode::Literal(LiteralValueNode::Int(1)),
          ExpressionNode::Literal(LiteralValueNode::Int(2)),
          ExpressionNode::Literal(LiteralValueNode::Int(3)),
        ])),
        body: vec![],
      }
    );
  }
}
