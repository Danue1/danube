use super::*;

pub(super) fn parse_for_node(s: Tokens) -> ParseResult<ForNode> {
  map(
    tuple((
      parse_keyword(Keyword::For),
      parse_immutablity_kind,
      parse_pattern_kind,
      parse_keyword(Keyword::In),
      parse_expression_kind,
      parse_block_node,
    )),
    |(_, immutablity, pattern, _, iteration, block)| ForNode {
      immutablity,
      pattern,
      iteration: Box::new(iteration),
      block,
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
        immutablity: ImmutablityKind::Yes,
        pattern: PatternKind::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "foo".to_owned()
          }]
        }),
        iteration: Box::new(ExpressionKind::Array(vec![
          ExpressionKind::Literal(LiteralKind::Int(1)),
          ExpressionKind::Literal(LiteralKind::Int(2)),
          ExpressionKind::Literal(LiteralKind::Int(3)),
        ])),
        block: BlockNode {
          statement_list: vec![],
        },
      }
    );
  }
}
