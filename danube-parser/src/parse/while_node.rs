use super::*;

pub(super) fn parse_while_node(s: Tokens) -> ParseResult<WhileNode> {
  map(
    tuple((
      parse_keyword(Keyword::While),
      parse_condition_node,
      parse_block_node,
    )),
    |(_, condition, block)| WhileNode { condition, block },
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
  fn value() {
    let source = "while true { }";
    assert_eq!(
      compile(source),
      WhileNode {
        condition: ConditionNode {
          pattern: None,
          value: Box::new(ExpressionNode::Literal(LiteralValueNode::Bool(true)))
        },
        block: BlockNode {
          statement_list: vec![]
        }
      }
    );
  }

  #[test]
  fn let_value() {
    let source = "while let foo = true { }";
    assert_eq!(
      compile(source),
      WhileNode {
        condition: ConditionNode {
          pattern: Some((
            Immutablity::Yes,
            PatternNode::Path(PathNode {
              ident_list: vec![IdentNode {
                raw: "foo".to_owned(),
              }]
            })
          )),
          value: Box::new(ExpressionNode::Literal(LiteralValueNode::Bool(true)))
        },
        block: BlockNode {
          statement_list: vec![]
        }
      }
    );
  }

  #[test]
  fn let_mut_value() {
    let source = "while let mut foo = true { }";
    assert_eq!(
      compile(source),
      WhileNode {
        condition: ConditionNode {
          pattern: Some((
            Immutablity::Nope,
            PatternNode::Path(PathNode {
              ident_list: vec![IdentNode {
                raw: "foo".to_owned(),
              }]
            })
          )),
          value: Box::new(ExpressionNode::Literal(LiteralValueNode::Bool(true)))
        },
        block: BlockNode {
          statement_list: vec![]
        }
      }
    );
  }
}
