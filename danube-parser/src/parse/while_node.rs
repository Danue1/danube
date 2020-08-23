use super::*;

pub(super) fn parse_while_node(s: Tokens) -> ParseResult<WhileNode> {
  map(
    tuple((
      parse_keyword(Keyword::While),
      parse_expression_node,
      parse_block_node,
    )),
    |(_, condition, block)| WhileNode {
      condition: Box::new(condition),
      block,
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
        condition: Box::new(ExpressionNode::Literal(LiteralValueNode::Bool(true))),
        block: BlockNode {
          statement_list: vec![]
        }
      }
    );
  }
}
