use super::*;

pub(super) fn parse_conditional_node(s: Tokens) -> ParseResult<ConditionalNode> {
  map(
    tuple((
      parse_keyword(Keyword::If),
      tuple((parse_expression_node, parse_block_node)),
      many0(map(
        tuple((
          parse_keyword(Keyword::Else),
          parse_keyword(Keyword::If),
          tuple((parse_expression_node, parse_block_node)),
        )),
        |(_, _, block)| block,
      )),
      opt(map(
        tuple((parse_keyword(Keyword::Else), parse_block_node)),
        |(_, block)| block,
      )),
    )),
    |(_, if_conditional_branch, else_if_conditional_branch, else_conditional_branch)| {
      ConditionalNode {
        main_branch: Box::new(if_conditional_branch),
        branch_list: else_if_conditional_branch,
        other: else_conditional_branch,
      }
    },
  )(s)
}

#[cfg(test)]
mod tests {
  use super::*;

  fn compile(s: &str) -> ConditionalNode {
    let (_, token_list) = lex(s).unwrap();
    match parse_conditional_node(Tokens::new(&token_list)) {
      Ok((_, node)) => node,
      Err(error) => {
        dbg!(error);
        panic!()
      }
    }
  }

  #[test]
  fn if_branch_else_branch() {
    let source = "if true { } else { }";
    assert_eq!(
      compile(source),
      ConditionalNode {
        main_branch: Box::new((
          ExpressionNode::Literal(LiteralValueNode::Bool(true)),
          BlockNode {
            statement_list: vec![]
          },
        )),
        branch_list: vec![],
        other: Some(BlockNode {
          statement_list: vec![]
        }),
      }
    );
  }

  #[test]
  fn if_branch_else_if_branch_else_branch() {
    let source = "if true { } else if true { } else { }";
    assert_eq!(
      compile(source),
      ConditionalNode {
        main_branch: Box::new((
          ExpressionNode::Literal(LiteralValueNode::Bool(true)),
          BlockNode {
            statement_list: vec![]
          },
        )),
        branch_list: vec![(
          ExpressionNode::Literal(LiteralValueNode::Bool(true)),
          BlockNode {
            statement_list: vec![]
          },
        )],
        other: Some(BlockNode {
          statement_list: vec![]
        }),
      }
    );
  }
}
