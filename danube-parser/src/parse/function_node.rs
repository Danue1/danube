use super::*;

pub(super) fn parse_function_node(s: Tokens) -> ParseResult<FunctionNode> {
  map(
    tuple((
      opt(parse_visibility_kind),
      opt(parse_keyword(Keyword::Async)),
      parse_keyword(Keyword::Function),
      parse_ident_node,
      opt(parse_generic_node),
      parse_function_argument_list,
      opt(parse_function_type),
      parse_function_body,
    )),
    |(visibility, is_async, _, ident, generic, (self_type, argument_list), return_type, block)| {
      FunctionNode {
        visibility,
        is_async: is_async.is_some(),
        ident,
        generic,
        self_type,
        argument_list,
        return_type,
        block,
      }
    },
  )(s)
}

fn parse_function_argument_list(
  s: Tokens,
) -> ParseResult<(Option<ImmutablityKind>, Vec<FunctionArgumentNode>)> {
  map(
    tuple((
      parse_symbol(Symbol::LeftParens),
      opt(map(
        tuple((
          parse_immutablity_kind,
          parse_keyword(Keyword::VariableSelf),
          opt(parse_symbol(Symbol::Comma)),
        )),
        |(self_type, _, _)| self_type,
      )),
      separated_list(parse_symbol(Symbol::Comma), parse_function_argument_node),
      opt(parse_symbol(Symbol::Comma)),
      parse_symbol(Symbol::RightParens),
    )),
    |(_, self_type, argument_list, _, _)| (self_type, argument_list),
  )(s)
}

fn parse_function_type(s: Tokens) -> ParseResult<TypeKind> {
  map(
    tuple((parse_symbol(Symbol::ReturnArrow), parse_type_kind)),
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
      parse_expression_kind,
      parse_symbol(Symbol::Semicolon),
    )),
    |(_, expression, _)| BlockNode {
      statement_list: vec![StatementKind::ExpressionKind(expression)],
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
        self_type: None,
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
        self_type: None,
        argument_list: vec![FunctionArgumentNode {
          immutablity: ImmutablityKind::Yes,
          ident: IdentNode {
            raw: "bar".to_owned()
          },
          ty: TypeKind::Path(
            ImmutablityKind::Yes,
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
        self_type: None,
        argument_list: vec![
          FunctionArgumentNode {
            immutablity: ImmutablityKind::Yes,
            ident: IdentNode {
              raw: "bar".to_owned()
            },
            ty: TypeKind::Path(
              ImmutablityKind::Yes,
              PathNode {
                ident_list: vec![IdentNode {
                  raw: "Bar".to_owned()
                }]
              }
            )
          },
          FunctionArgumentNode {
            immutablity: ImmutablityKind::Yes,
            ident: IdentNode {
              raw: "baz".to_owned()
            },
            ty: TypeKind::Path(
              ImmutablityKind::Yes,
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
        self_type: None,
        argument_list: vec![FunctionArgumentNode {
          immutablity: ImmutablityKind::Nope,
          ident: IdentNode {
            raw: "bar".to_owned()
          },
          ty: TypeKind::Path(
            ImmutablityKind::Yes,
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
        self_type: None,
        argument_list: vec![FunctionArgumentNode {
          immutablity: ImmutablityKind::Yes,
          ident: IdentNode {
            raw: "bar".to_owned()
          },
          ty: TypeKind::Path(
            ImmutablityKind::Nope,
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
        self_type: None,
        argument_list: vec![],
        return_type: None,
        block: BlockNode {
          statement_list: vec![StatementKind::ExpressionKind(ExpressionKind::Conditional(
            ConditionalNode {
              main_branch: (
                ConditionNode {
                  pattern: None,
                  value: Box::new(ExpressionKind::Path(PathNode {
                    ident_list: vec![IdentNode {
                      raw: "true".to_owned()
                    }]
                  })),
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
        self_type: None,
        argument_list: vec![],
        return_type: None,
        block: BlockNode {
          statement_list: vec![StatementKind::ExpressionKind(ExpressionKind::Conditional(
            ConditionalNode {
              main_branch: (
                ConditionNode {
                  pattern: None,
                  value: Box::new(ExpressionKind::Path(PathNode {
                    ident_list: vec![IdentNode {
                      raw: "true".to_owned()
                    }]
                  }))
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
        self_type: None,
        argument_list: vec![],
        return_type: None,
        block: BlockNode {
          statement_list: vec![StatementKind::ExpressionKind(ExpressionKind::PatternMatch(
            PatternMatchNode {
              condition: Box::new(ExpressionKind::Path(PathNode {
                ident_list: vec![IdentNode {
                  raw: "true".to_owned()
                }]
              })),
              branch_list: vec![(
                vec![PatternKind::Path(PathNode {
                  ident_list: vec![IdentNode {
                    raw: "true".to_owned()
                  }]
                })],
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
        self_type: None,
        argument_list: vec![],
        return_type: None,
        block: BlockNode {
          statement_list: vec![StatementKind::Let(Box::new(LetNode {
            immutablity: ImmutablityKind::Nope,
            pattern: PatternKind::Path(PathNode {
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
            value: Some(ExpressionKind::Path(PathNode {
              ident_list: vec![IdentNode {
                raw: "true".to_owned()
              }]
            })),
          }))]
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
        self_type: None,
        argument_list: vec![],
        return_type: None,
        block: BlockNode {
          statement_list: vec![]
        },
      }
    );
  }

  #[test]
  fn shorthand_function() {
    let source = "fn foo() = 1;";
    assert_eq!(
      compile(source),
      FunctionNode {
        visibility: None,
        is_async: false,
        ident: IdentNode {
          raw: "foo".to_owned()
        },
        generic: None,
        self_type: None,
        argument_list: vec![],
        return_type: None,
        block: BlockNode {
          statement_list: vec![StatementKind::ExpressionKind(ExpressionKind::Literal(
            LiteralKind::Int(1)
          ))]
        },
      }
    );
  }
}
