use super::*;

pub(super) fn parse_let_mut_node(s: Tokens) -> ParseResult<LetMutNode> {
  map(
    tuple((
      parse_keyword(Keyword::Let),
      parse_keyword(Keyword::Mut),
      parse_pattern_node,
      opt(map(
        tuple((parse_symbol(Symbol::Colon), parse_type_node)),
        |(_, ty)| ty,
      )),
      parse_symbol(Symbol::Assign),
      parse_expression_node,
      parse_symbol(Symbol::Semicolon),
    )),
    |(_, _, pattern, ty, _, value, _)| LetMutNode { pattern, ty, value },
  )(s)
}

#[cfg(test)]
mod tests {
  use super::*;

  fn compile(s: &str) -> LetMutNode {
    let (_, token_list) = lex(s).unwrap();
    match parse_let_mut_node(Tokens::new(&token_list)) {
      Ok((_, node)) => node,
      Err(error) => {
        dbg!(error);
        panic!()
      }
    }
  }

  #[test]
  fn untyped() {
    let source = "let mut foo = true;";
    assert_eq!(
      compile(source),
      LetMutNode {
        pattern: PatternNode::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "foo".to_owned()
          }]
        }),
        ty: None,
        value: ExpressionNode::Literal(LiteralValueNode::Bool(true)),
      }
    );
  }

  #[test]
  fn typed() {
    let source = "let mut foo: bool = true;";
    assert_eq!(
      compile(source),
      LetMutNode {
        pattern: PatternNode::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "foo".to_owned()
          }]
        }),
        ty: Some(TypeNode::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "bool".to_owned()
          }]
        })),
        value: ExpressionNode::Literal(LiteralValueNode::Bool(true)),
      }
    );
  }

  #[test]
  fn pattern_unnamed() {
    let source = "let mut Foo::Bar = true;";
    assert_eq!(
      compile(source),
      LetMutNode {
        pattern: PatternNode::Path(PathNode {
          ident_list: vec![
            IdentNode {
              raw: "Foo".to_owned()
            },
            IdentNode {
              raw: "Bar".to_owned()
            }
          ]
        }),
        ty: None,
        value: ExpressionNode::Literal(LiteralValueNode::Bool(true)),
      }
    );
  }
}
