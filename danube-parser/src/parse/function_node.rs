use super::*;

pub(super) fn parse_function_node(s: Tokens) -> ParseResult<FunctionNode> {
  map(
    tuple((
      opt(parse_visibility),
      opt(parse_keyword(Keyword::Async)),
      parse_keyword(Keyword::Function),
      parse_ident_node,
      opt(parse_generic_node),
      parse_function_argument_list,
      opt(parse_function_type),
      parse_function_body,
    )),
    |(visibility, is_async, _, ident, generic, argument_list, return_type, block)| FunctionNode {
      visibility,
      is_async: is_async.is_some(),
      ident,
      generic,
      argument_list,
      return_type,
      block,
    },
  )(s)
}

fn parse_function_argument_list(s: Tokens) -> ParseResult<Vec<FunctionArgumentNode>> {
  map(
    tuple((
      parse_symbol(Symbol::LeftParens),
      separated_list(parse_symbol(Symbol::Comma), parse_function_argument_node),
      opt(parse_symbol(Symbol::Comma)),
      parse_symbol(Symbol::RightParens),
    )),
    |(_, argument_list, _, _)| argument_list,
  )(s)
}

fn parse_function_type(s: Tokens) -> ParseResult<TypeNode> {
  map(
    tuple((parse_symbol(Symbol::ReturnArrow), parse_type_node)),
    |(_, ty)| ty,
  )(s)
}

fn parse_function_body(s: Tokens) -> ParseResult<BlockNode> {
  alt((parse_function_body_shortcut, parse_block_node))(s)
}

fn parse_function_body_shortcut(s: Tokens) -> ParseResult<BlockNode> {
  map(
    tuple((
      parse_symbol(Symbol::Assign),
      parse_expression_node,
      parse_symbol(Symbol::Semicolon),
    )),
    |(_, expression, _)| BlockNode {
      statement_list: vec![StatementNode::Expression(expression)],
    },
  )(s)
}

#[cfg(test)]
mod tests {
  use super::*;

  fn compile(s: &str) -> FunctionNode {
    let (_, token_list) = lex(s).unwrap();
    match parse_function_node(Tokens::new(&token_list)) {
      Ok((_, node)) => node,
      Err(error) => {
        dbg!(error);
        panic!()
      }
    }
  }

  #[test]
  fn no_argument() {
    let source = "fn foo() { }";
    assert_eq!(
      compile(source),
      FunctionNode {
        visibility: None,
        is_async: false,
        ident: IdentNode {
          raw: "foo".to_owned()
        },
        generic: None,
        argument_list: vec![],
        return_type: None,
        block: BlockNode {
          statement_list: vec![]
        },
      }
    );
  }

  #[test]
  fn a_argument() {
    let source = "fn foo(bar: Bar) { }";
    assert_eq!(
      compile(source),
      FunctionNode {
        visibility: None,
        is_async: false,
        ident: IdentNode {
          raw: "foo".to_owned()
        },
        generic: None,
        argument_list: vec![FunctionArgumentNode {
          immutablity: Immutablity::Yes,
          ident: IdentNode {
            raw: "bar".to_owned()
          },
          ty: TypeNode::Path(
            Immutablity::Yes,
            PathNode {
              ident_list: vec![IdentNode {
                raw: "Bar".to_owned()
              }]
            }
          )
        }],
        return_type: None,
        block: BlockNode {
          statement_list: vec![]
        },
      }
    );
  }

  #[test]
  fn two_argument() {
    let source = "fn foo(bar: Bar, baz: Baz) { }";
    assert_eq!(
      compile(source),
      FunctionNode {
        visibility: None,
        is_async: false,
        ident: IdentNode {
          raw: "foo".to_owned()
        },
        generic: None,
        argument_list: vec![
          FunctionArgumentNode {
            immutablity: Immutablity::Yes,
            ident: IdentNode {
              raw: "bar".to_owned()
            },
            ty: TypeNode::Path(
              Immutablity::Yes,
              PathNode {
                ident_list: vec![IdentNode {
                  raw: "Bar".to_owned()
                }]
              }
            )
          },
          FunctionArgumentNode {
            immutablity: Immutablity::Yes,
            ident: IdentNode {
              raw: "baz".to_owned()
            },
            ty: TypeNode::Path(
              Immutablity::Yes,
              PathNode {
                ident_list: vec![IdentNode {
                  raw: "Baz".to_owned()
                }]
              }
            )
          }
        ],
        return_type: None,
        block: BlockNode {
          statement_list: vec![]
        },
      }
    );
  }

  #[test]
  fn a_mutable_argument() {
    let source = "fn foo(mut bar: Bar) { }";
    assert_eq!(
      compile(source),
      FunctionNode {
        visibility: None,
        is_async: false,
        ident: IdentNode {
          raw: "foo".to_owned()
        },
        generic: None,
        argument_list: vec![FunctionArgumentNode {
          immutablity: Immutablity::Nope,
          ident: IdentNode {
            raw: "bar".to_owned()
          },
          ty: TypeNode::Path(
            Immutablity::Yes,
            PathNode {
              ident_list: vec![IdentNode {
                raw: "Bar".to_owned()
              }]
            }
          )
        }],
        return_type: None,
        block: BlockNode {
          statement_list: vec![]
        },
      }
    );
  }

  #[test]
  fn a_mutable_type_argument() {
    let source = "fn foo(bar: mut Bar) { }";
    assert_eq!(
      compile(source),
      FunctionNode {
        visibility: None,
        is_async: false,
        ident: IdentNode {
          raw: "foo".to_owned()
        },
        generic: None,
        argument_list: vec![FunctionArgumentNode {
          immutablity: Immutablity::Yes,
          ident: IdentNode {
            raw: "bar".to_owned()
          },
          ty: TypeNode::Path(
            Immutablity::Nope,
            PathNode {
              ident_list: vec![IdentNode {
                raw: "Bar".to_owned()
              }]
            }
          )
        }],
        return_type: None,
        block: BlockNode {
          statement_list: vec![]
        },
      }
    );
  }

  #[test]
  fn function_conditional_if() {
    let source = "fn foo() {
      if true { }
    }";
    assert_eq!(
      compile(source),
      FunctionNode {
        visibility: None,
        is_async: false,
        ident: IdentNode {
          raw: "foo".to_owned()
        },
        generic: None,
        argument_list: vec![],
        return_type: None,
        block: BlockNode {
          statement_list: vec![StatementNode::Expression(ExpressionNode::Conditional(
            ConditionalNode {
              main_branch: (
                ConditionNode {
                  pattern: None,
                  value: Box::new(ExpressionNode::Literal(LiteralValueNode::Bool(true))),
                },
                BlockNode {
                  statement_list: vec![]
                },
              ),
              branch_list: vec![],
              other: None
            }
          ))]
        },
      }
    );
  }

  #[test]
  fn function_conditional_else() {
    let source = "fn foo() {
      if true { } else { }
    }";
    assert_eq!(
      compile(source),
      FunctionNode {
        visibility: None,
        is_async: false,
        ident: IdentNode {
          raw: "foo".to_owned()
        },
        generic: None,
        argument_list: vec![],
        return_type: None,
        block: BlockNode {
          statement_list: vec![StatementNode::Expression(ExpressionNode::Conditional(
            ConditionalNode {
              main_branch: (
                ConditionNode {
                  pattern: None,
                  value: Box::new(ExpressionNode::Literal(LiteralValueNode::Bool(true)))
                },
                BlockNode {
                  statement_list: vec![]
                },
              ),
              branch_list: vec![],
              other: Some(BlockNode {
                statement_list: vec![]
              }),
            }
          ))]
        },
      }
    );
  }

  #[test]
  fn function_pattern_match() {
    let source = "fn foo() {
      match true {
        true => { },
      }
    }";
    assert_eq!(
      compile(source),
      FunctionNode {
        visibility: None,
        is_async: false,
        ident: IdentNode {
          raw: "foo".to_owned()
        },
        generic: None,
        argument_list: vec![],
        return_type: None,
        block: BlockNode {
          statement_list: vec![StatementNode::Expression(ExpressionNode::PatternMatch(
            PatternMatchNode {
              condition: Box::new(ExpressionNode::Literal(LiteralValueNode::Bool(true))),
              branch_list: vec![(
                vec![PatternNode::Literal(LiteralValueNode::Bool(true))],
                BlockNode {
                  statement_list: vec![]
                },
              )],
            }
          ))]
        },
      }
    );
  }

  #[test]
  fn function_expression_let_mut_pattern() {
    let source = "fn foo() {
      let mut Foo::Bar = true;
    }";
    assert_eq!(
      compile(source),
      FunctionNode {
        visibility: None,
        is_async: false,
        ident: IdentNode {
          raw: "foo".to_owned()
        },
        generic: None,
        argument_list: vec![],
        return_type: None,
        block: BlockNode {
          statement_list: vec![StatementNode::Let(LetNode {
            immutablity: Immutablity::Nope,
            pattern: PatternNode::Path(PathNode {
              ident_list: vec![
                IdentNode {
                  raw: "Foo".to_owned()
                },
                IdentNode {
                  raw: "Bar".to_owned()
                },
              ]
            }),
            ty: None,
            value: ExpressionNode::Literal(LiteralValueNode::Bool(true)),
          })]
        },
      }
    );
  }

  #[test]
  fn async_function() {
    let source = "async fn foo() { }";
    assert_eq!(
      compile(source),
      FunctionNode {
        visibility: None,
        is_async: true,
        ident: IdentNode {
          raw: "foo".to_owned()
        },
        generic: None,
        argument_list: vec![],
        return_type: None,
        block: BlockNode {
          statement_list: vec![]
        },
      }
    );
  }
}
