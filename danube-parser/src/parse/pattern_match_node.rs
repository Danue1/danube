use super::*;

pub(super) fn parse_pattern_match_node(s: Tokens) -> ParseResult<PatternMatchNode> {
  map(
    tuple((
      parse_keyword(Keyword::Match),
      parse_expression_node,
      parse_symbol(Symbol::LeftBrace),
      separated_nonempty_list(
        parse_symbol(Symbol::Comma),
        map(
          tuple((
            separated_nonempty_list(parse_symbol(Symbol::BitOr), parse_pattern_node),
            parse_symbol(Symbol::BranchArrow),
            parse_body,
          )),
          |(pattern, _, body)| (pattern, body),
        ),
      ),
      opt(parse_symbol(Symbol::Comma)),
      parse_symbol(Symbol::RightBrace),
    )),
    |(_, condition, _, branch_list, _, _)| PatternMatchNode {
      condition: Box::new(condition),
      branch_list,
    },
  )(s)
}

fn parse_body(s: Tokens) -> ParseResult<BlockNode> {
  alt((parse_block_node, parse_body_shortcut))(s)
}

fn parse_body_shortcut(s: Tokens) -> ParseResult<BlockNode> {
  map(parse_statement_node, |statement| BlockNode {
    statement_list: vec![statement],
  })(s)
}

#[cfg(test)]
mod tests {
  use super::*;

  fn compile(s: &str) -> PatternMatchNode {
    let (_, token_list) = lex(s).unwrap();
    match parse_pattern_match_node(Tokens::new(&token_list)) {
      Ok((_, node)) => node,
      Err(error) => {
        dbg!(error);
        panic!()
      }
    }
  }

  #[test]
  fn a_shortcut_branch() {
    let source = "match true {
      true => true,
    }";
    assert_eq!(
      compile(source),
      PatternMatchNode {
        condition: Box::new(ExpressionNode::Literal(LiteralValueNode::Bool(true))),
        branch_list: vec![(
          vec![PatternNode::Literal(LiteralValueNode::Bool(true))],
          BlockNode {
            statement_list: vec![StatementNode::Expression(ExpressionNode::Literal(
              LiteralValueNode::Bool(true)
            ))]
          },
        )],
      }
    );
  }

  #[test]
  fn two_shortcut_branch() {
    let source = "match true {
      true => true,
      false => false,
    }";
    assert_eq!(
      compile(source),
      PatternMatchNode {
        condition: Box::new(ExpressionNode::Literal(LiteralValueNode::Bool(true))),
        branch_list: vec![
          (
            vec![PatternNode::Literal(LiteralValueNode::Bool(true))],
            BlockNode {
              statement_list: vec![StatementNode::Expression(ExpressionNode::Literal(
                LiteralValueNode::Bool(true)
              ))]
            }
          ),
          (
            vec![PatternNode::Literal(LiteralValueNode::Bool(false))],
            BlockNode {
              statement_list: vec![StatementNode::Expression(ExpressionNode::Literal(
                LiteralValueNode::Bool(false)
              ))]
            },
          )
        ],
      }
    );
  }

  #[test]
  fn a_longcut_branch() {
    let source = "match true {
      true => { },
    }";
    assert_eq!(
      compile(source),
      PatternMatchNode {
        condition: Box::new(ExpressionNode::Literal(LiteralValueNode::Bool(true))),
        branch_list: vec![(
          vec![PatternNode::Literal(LiteralValueNode::Bool(true))],
          BlockNode {
            statement_list: vec![]
          },
        )],
      }
    );
  }

  #[test]
  fn two_longcut_branch() {
    let source = "match true {
      true => { },
      false => { },
    }";
    assert_eq!(
      compile(source),
      PatternMatchNode {
        condition: Box::new(ExpressionNode::Literal(LiteralValueNode::Bool(true))),
        branch_list: vec![
          (
            vec![PatternNode::Literal(LiteralValueNode::Bool(true))],
            BlockNode {
              statement_list: vec![]
            },
          ),
          (
            vec![PatternNode::Literal(LiteralValueNode::Bool(false))],
            BlockNode {
              statement_list: vec![]
            },
          )
        ],
      }
    );
  }

  #[test]
  fn two_pattern() {
    let source = "match true {
      true | false => { },
    }";
    assert_eq!(
      compile(source),
      PatternMatchNode {
        condition: Box::new(ExpressionNode::Literal(LiteralValueNode::Bool(true))),
        branch_list: vec![(
          vec![
            PatternNode::Literal(LiteralValueNode::Bool(true)),
            PatternNode::Literal(LiteralValueNode::Bool(false))
          ],
          BlockNode {
            statement_list: vec![]
          },
        )],
      }
    );
  }
}
