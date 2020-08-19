use super::*;

pub(super) fn parse_let_node(s: Tokens) -> ParseResult<LetNode> {
  map(
    tuple((
      parse_keyword(Keyword::Let),
      parse_pattern_node,
      opt(map(
        tuple((parse_symbol(Symbol::Colon), parse_type_node)),
        |(_, ty)| ty,
      )),
      parse_symbol(Symbol::Assign),
      parse_expression_node,
      parse_symbol(Symbol::Semicolon),
    )),
    |(_, pattern, ty, _, value, _)| LetNode { pattern, ty, value },
  )(s)
}

#[cfg(test)]
mod tests {
  use super::*;

  fn compile(s: &str) -> LetNode {
    let (_, token_list) = lex(s).unwrap();
    match parse_let_node(Tokens::new(&token_list)) {
      Ok((_, node)) => node,
      Err(error) => {
        dbg!(error);
        panic!()
      }
    }
  }

  #[test]
  fn untyped() {
    let source = "let foo = true;";
    assert_eq!(
      compile(source),
      LetNode {
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
    let source = "let foo: bool = true;";
    assert_eq!(
      compile(source),
      LetNode {
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
  fn unnamed_implicit() {
    let source = "let (foo) = true;";
    assert_eq!(
      compile(source),
      LetNode {
        pattern: PatternNode::UnnamedStruct(UnnamedStructNode {
          path: None,
          field_list: vec![PatternNode::Path(PathNode {
            ident_list: vec![IdentNode {
              raw: "foo".to_owned()
            }]
          })]
        }),
        ty: None,
        value: ExpressionNode::Literal(LiteralValueNode::Bool(true)),
      }
    );
  }

  #[test]
  fn unnamed_single() {
    let source = "let Foo(foo) = true;";
    assert_eq!(
      compile(source),
      LetNode {
        pattern: PatternNode::UnnamedStruct(UnnamedStructNode {
          path: Some(PathNode {
            ident_list: vec![IdentNode {
              raw: "Foo".to_owned()
            }]
          }),
          field_list: vec![PatternNode::Path(PathNode {
            ident_list: vec![IdentNode {
              raw: "foo".to_owned()
            }]
          })]
        }),
        ty: None,
        value: ExpressionNode::Literal(LiteralValueNode::Bool(true)),
      }
    );
  }

  #[test]
  fn unnamed_double() {
    let source = "let Foo(foo, bar) = true;";
    assert_eq!(
      compile(source),
      LetNode {
        pattern: PatternNode::UnnamedStruct(UnnamedStructNode {
          path: Some(PathNode {
            ident_list: vec![IdentNode {
              raw: "Foo".to_owned()
            }]
          }),
          field_list: vec![
            PatternNode::Path(PathNode {
              ident_list: vec![IdentNode {
                raw: "foo".to_owned()
              }]
            }),
            PatternNode::Path(PathNode {
              ident_list: vec![IdentNode {
                raw: "bar".to_owned()
              }]
            })
          ]
        }),
        ty: None,
        value: ExpressionNode::Literal(LiteralValueNode::Bool(true)),
      }
    );
  }

  #[test]
  fn named_single_implicit() {
    let source = "let { foo } = true;";
    assert_eq!(
      compile(source),
      LetNode {
        pattern: PatternNode::NamedStruct(NamedStructNode {
          path: None,
          field_list: vec![FieldNode {
            ident: IdentNode {
              raw: "foo".to_owned()
            },
            pattern: None
          }]
        }),
        ty: None,
        value: ExpressionNode::Literal(LiteralValueNode::Bool(true)),
      }
    );
  }

  #[test]
  fn named_single() {
    let source = "let Foo { foo } = true;";
    assert_eq!(
      compile(source),
      LetNode {
        pattern: PatternNode::NamedStruct(NamedStructNode {
          path: Some(PathNode {
            ident_list: vec![IdentNode {
              raw: "Foo".to_owned()
            }]
          }),
          field_list: vec![FieldNode {
            ident: IdentNode {
              raw: "foo".to_owned()
            },
            pattern: None
          }]
        }),
        ty: None,
        value: ExpressionNode::Literal(LiteralValueNode::Bool(true)),
      }
    );
  }

  #[test]
  fn named_double() {
    let source = "let Foo { foo, bar } = true;";
    assert_eq!(
      compile(source),
      LetNode {
        pattern: PatternNode::NamedStruct(NamedStructNode {
          path: Some(PathNode {
            ident_list: vec![IdentNode {
              raw: "Foo".to_owned()
            }]
          }),
          field_list: vec![
            FieldNode {
              ident: IdentNode {
                raw: "foo".to_owned()
              },
              pattern: None
            },
            FieldNode {
              ident: IdentNode {
                raw: "bar".to_owned()
              },
              pattern: None
            }
          ]
        }),
        ty: None,
        value: ExpressionNode::Literal(LiteralValueNode::Bool(true)),
      }
    );
  }
}
