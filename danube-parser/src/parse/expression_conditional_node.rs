use super::*;

pub(super) fn parse_expression_conditional_node(
  s: Tokens,
) -> ParseResult<ExpressionConditionalNode> {
  map(
    tuple((
      parse_keyword(Keyword::If),
      tuple((parse_expression_node, parse_branch_body)),
      many0(map(
        tuple((
          parse_keyword(Keyword::Else),
          parse_keyword(Keyword::If),
          tuple((parse_expression_node, parse_branch_body)),
        )),
        |(_, _, conditional_branch)| conditional_branch,
      )),
      map(
        tuple((parse_keyword(Keyword::Else), parse_branch_body)),
        |(_, statement_list)| statement_list,
      ),
    )),
    |(_, if_conditional_branch, else_if_conditional_branch, else_conditional_branch)| {
      ExpressionConditionalNode {
        main_branch: Box::new(if_conditional_branch),
        branch_list: else_if_conditional_branch,
        other: else_conditional_branch,
      }
    },
  )(s)
}

fn parse_branch_body(s: Tokens) -> ParseResult<Vec<StatementNode>> {
  map(
    tuple((
      parse_symbol(Symbol::LeftBrace),
      many0(parse_statement_node),
      parse_symbol(Symbol::RightBrace),
    )),
    |(_, statement_list, _)| statement_list,
  )(s)
}

#[cfg(test)]
mod tests {
  use super::*;

  fn compile(s: &str) -> ExpressionConditionalNode {
    let (_, token_list) = lex(s).unwrap();
    match parse_expression_conditional_node(Tokens::new(&token_list)) {
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
      ExpressionConditionalNode {
        main_branch: Box::new((
          ExpressionNode::Literal(LiteralValueNode::Bool(true)),
          vec![]
        )),
        branch_list: vec![],
        other: vec![],
      }
    );
  }

  #[test]
  fn if_branch_else_if_branch_else_branch() {
    let source = "if true { } else if true { } else { }";
    assert_eq!(
      compile(source),
      ExpressionConditionalNode {
        main_branch: Box::new((
          ExpressionNode::Literal(LiteralValueNode::Bool(true)),
          vec![]
        )),
        branch_list: vec![(
          ExpressionNode::Literal(LiteralValueNode::Bool(true)),
          vec![]
        )],
        other: vec![],
      }
    );
  }
}
