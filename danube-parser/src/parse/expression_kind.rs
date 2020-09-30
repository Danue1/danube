use super::*;

pub(super) fn parse_expression_kind(s: Tokens) -> ParseResult<ExpressionKind> {
  let (s, node) = parse_prefixable_expression_node(s)?;

  parse_postfix_expression_node(s, Precedence::Lowest, node)
}

fn parse_prefixable_expression_node(s: Tokens) -> ParseResult<ExpressionKind> {
  alt((parse_prefix_expression_node, parse_atomic_expression_node))(s)
}

fn parse_atomic_expression_node(s: Tokens) -> ParseResult<ExpressionKind> {
  alt((
    map(parse_expression_struct_node, ExpressionKind::Struct),
    map(parse_expression_tuple_node, ExpressionKind::Tuple),
    map(parse_path_node, ExpressionKind::Path),
    map(parse_conditional_node, ExpressionKind::Conditional),
    map(parse_loop_node, ExpressionKind::Loop),
    map(parse_while_node, ExpressionKind::While),
    map(parse_for_node, ExpressionKind::For),
    map(parse_pattern_match_node, ExpressionKind::PatternMatch),
    map(parse_closure_node, ExpressionKind::Closure),
    map(parse_literal_kind, ExpressionKind::Literal),
    map(parse_break, |_| ExpressionKind::Break),
    map(parse_continue, |_| ExpressionKind::Continue),
    map(parse_return_node, ExpressionKind::Return),
    map(parse_array, ExpressionKind::Array),
    map(parse_block_node, ExpressionKind::Block),
  ))(s)
}

fn parse_expression_struct_node(s: Tokens) -> ParseResult<ExpressionKindStructNode> {
  map(parse_expression_field_list, |(field_list, rest)| {
    ExpressionKindStructNode {
      path: None,
      field_list,
      rest,
    }
  })(s)
}

fn parse_expression_tuple_node(s: Tokens) -> ParseResult<TupleNode> {
  map(parse_tuple_operator, |argument_list| TupleNode {
    field: None,
    argument_list,
  })(s)
}

fn parse_prefix_expression_node(s: Tokens) -> ParseResult<ExpressionKind> {
  map(
    tuple((parse_unary_operator_kind, parse_expression_kind)),
    |(kind, value)| {
      ExpressionKind::UnaryOperator(UnaryOperatorNode {
        kind,
        value: Box::new(value),
      })
    },
  )(s)
}

#[derive(PartialEq, PartialOrd)]
enum Precedence {
  Lowest,
  Sum,
  Mul,
  Comparison,
  LazyBoolean,
  ChainArrow,
}

fn parse_postfix_expression_node(
  s: Tokens,
  precedence: Precedence,
  lhs: ExpressionKind,
) -> ParseResult<ExpressionKind> {
  match parse_operator_kind(s.clone()) {
    Ok((s, OperatorKind::Await)) => {
      let node = ExpressionKind::Await(Box::new(lhs));

      parse_postfix_expression_node(s, precedence, node)
    }
    Ok((s, OperatorKind::Try)) => {
      let node = ExpressionKind::Try(Box::new(lhs));

      parse_postfix_expression_node(s, precedence, node)
    }
    Ok((s, OperatorKind::Tuple(argument_list))) => {
      let node = ExpressionKind::Tuple(TupleNode {
        field: Some(Box::new(lhs)),
        argument_list,
      });

      parse_postfix_expression_node(s, precedence, node)
    }
    Ok((s, OperatorKind::Index(rhs))) => {
      let node = ExpressionKind::Index(IndexNode {
        array: Box::new(lhs),
        index: Box::new(rhs),
      });

      parse_postfix_expression_node(s, precedence, node)
    }
    Ok((s, OperatorKind::Generic(generic_list))) => {
      let node = ExpressionKind::Generic(ExpressionGenericNode {
        expression: Box::new(lhs),
        generic_list,
      });

      parse_postfix_expression_node(s, precedence, node)
    }
    Ok((s, OperatorKind::Field(rhs))) => {
      let node = ExpressionKind::Field(ExpressionKindFieldNode {
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
      });

      parse_postfix_expression_node(s, precedence, node)
    }
    Ok((_, OperatorKind::InfixOperator(_))) => parse_infix_expression_node(s, precedence, lhs),
    _ => {
      if let ExpressionKind::Path(path) = lhs.clone() {
        if let Ok((s, (field_list, rest))) = parse_expression_field_list(s.clone()) {
          let node = ExpressionKind::Struct(ExpressionKindStructNode {
            path: Some(path),
            field_list,
            rest,
          });

          parse_postfix_expression_node(s, precedence, node)
        } else {
          Ok((s, lhs))
        }
      } else {
        Ok((s, lhs))
      }
    }
  }
}

fn parse_infix_expression_node(
  s: Tokens,
  precedence: Precedence,
  lhs: ExpressionKind,
) -> ParseResult<ExpressionKind> {
  if let Ok((ss, kind)) = parse_infix_operator_kind(s.clone()) {
    let right_precedence = infix_binding_power(kind.clone());
    if right_precedence < precedence {
      Ok((s, lhs))
    } else {
      let (s, rhs) = parse_prefixable_expression_node(ss)?;
      let (s, rhs) = parse_postfix_expression_node(s, right_precedence, rhs)?;
      let rhs = ExpressionKind::InfixOperator(InfixOperatorNode {
        kind,
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
      });

      parse_infix_expression_node(s, precedence, rhs)
    }
  } else {
    Ok((s, lhs))
  }
}

fn infix_binding_power(kind: InfixOperatorKind) -> Precedence {
  match kind {
    InfixOperatorKind::Add | InfixOperatorKind::Sub => Precedence::Sum,
    InfixOperatorKind::Mul
    | InfixOperatorKind::Div
    | InfixOperatorKind::Mod
    | InfixOperatorKind::BitAnd
    | InfixOperatorKind::BitOr
    | InfixOperatorKind::BitXor
    | InfixOperatorKind::BitLeft
    | InfixOperatorKind::BitRight => Precedence::Mul,
    InfixOperatorKind::Equal
    | InfixOperatorKind::NotEqual
    | InfixOperatorKind::LessThan
    | InfixOperatorKind::LessThanOrEqual
    | InfixOperatorKind::GreaterThan
    | InfixOperatorKind::GreaterThanOrEqual => Precedence::Comparison,
    InfixOperatorKind::And | InfixOperatorKind::Or => Precedence::LazyBoolean,
    InfixOperatorKind::ChainArrow => Precedence::ChainArrow,
  }
}

enum OperatorKind {
  Await,
  Try,
  Tuple(Vec<TupleArgumentNode>),
  Index(ExpressionKind),
  Generic(Vec<ExpressionGenericKind>),
  Field(IdentNode),
  InfixOperator(InfixOperatorKind),
}

fn parse_operator_kind(s: Tokens) -> ParseResult<OperatorKind> {
  alt((
    map(parse_await_operator, |_| OperatorKind::Await),
    map(parse_try_operator, |_| OperatorKind::Try),
    map(parse_tuple_operator, OperatorKind::Tuple),
    map(parse_index_operator, OperatorKind::Index),
    map(parse_generic_kind, OperatorKind::Generic),
    map(parse_field_operator, OperatorKind::Field),
    map(parse_infix_operator_kind, OperatorKind::InfixOperator),
  ))(s)
}

fn parse_tuple_operator(s: Tokens) -> ParseResult<Vec<TupleArgumentNode>> {
  map(
    tuple((
      parse_symbol(Symbol::LeftParens),
      separated_list(
        parse_symbol(Symbol::Comma),
        map(
          tuple((
            opt(map(
              tuple((parse_ident_node, parse_symbol(Symbol::Assign))),
              |(name, _)| name,
            )),
            parse_expression_kind,
          )),
          |(name, value)| TupleArgumentNode { name, value },
        ),
      ),
      opt(parse_symbol(Symbol::Comma)),
      parse_symbol(Symbol::RightParens),
    )),
    |(_, expression_list, _, _)| expression_list,
  )(s)
}

fn parse_index_operator(s: Tokens) -> ParseResult<ExpressionKind> {
  map(
    tuple((
      parse_symbol(Symbol::LeftBracket),
      parse_expression_kind,
      parse_symbol(Symbol::RightBracket),
    )),
    |(_, expression, _)| expression,
  )(s)
}

fn parse_generic_kind(s: Tokens) -> ParseResult<Vec<ExpressionGenericKind>> {
  map(
    tuple((
      parse_symbol(Symbol::LessThan),
      separated_nonempty_list(
        parse_symbol(Symbol::Comma),
        alt((
          map(
            tuple((
              parse_ident_node,
              parse_symbol(Symbol::Assign),
              parse_type_kind,
            )),
            |(ident, _, type_kind)| ExpressionGenericKind::Output(ident, type_kind),
          ),
          map(parse_type_kind, ExpressionGenericKind::Input),
        )),
      ),
      parse_symbol(Symbol::GreaterThan),
    )),
    |(_, generic_list, _)| generic_list,
  )(s)
}

fn parse_await_operator(s: Tokens) -> ParseResult<()> {
  map(
    tuple((parse_symbol(Symbol::Dot), parse_keyword(Keyword::Await))),
    |_| (),
  )(s)
}

fn parse_try_operator(s: Tokens) -> ParseResult<()> {
  map(parse_symbol(Symbol::Question), |_| ())(s)
}

fn parse_field_operator(s: Tokens) -> ParseResult<IdentNode> {
  map(
    tuple((parse_symbol(Symbol::Dot), parse_ident_node)),
    |(_, ident)| ident,
  )(s)
}

fn parse_expression_field_list(
  s: Tokens,
) -> ParseResult<(Vec<ExpressionKindStructField>, Option<Box<ExpressionKind>>)> {
  map(
    tuple((
      parse_symbol(Symbol::LeftBrace),
      separated_nonempty_list(
        parse_symbol(Symbol::Comma),
        tuple((
          parse_ident_node,
          opt(map(
            tuple((parse_symbol(Symbol::DoubleColon), parse_expression_kind)),
            |(_, expression)| expression,
          )),
        )),
      ),
      opt(parse_symbol(Symbol::Comma)),
      opt(map(
        tuple((parse_symbol(Symbol::RangeClose), parse_expression_kind)),
        |(_, expression)| Box::new(expression),
      )),
      parse_symbol(Symbol::RightBrace),
    )),
    |(_, expression_list, _, rest, _)| (expression_list, rest),
  )(s)
}

fn parse_break(s: Tokens) -> ParseResult<()> {
  map(parse_keyword(Keyword::Break), |_| ())(s)
}

fn parse_continue(s: Tokens) -> ParseResult<()> {
  map(parse_keyword(Keyword::Continue), |_| ())(s)
}

fn parse_array(s: Tokens) -> ParseResult<Vec<ExpressionKind>> {
  map(
    tuple((
      parse_symbol(Symbol::LeftBracket),
      separated_list(parse_symbol(Symbol::Comma), parse_expression_kind),
      opt(parse_symbol(Symbol::Comma)),
      parse_symbol(Symbol::RightBracket),
    )),
    |(_, expression_list, _, _)| expression_list,
  )(s)
}

#[cfg(test)]
mod tests {
  use super::*;

  fn compile(s: &str) -> ExpressionKind {
    let (_, token_list) = lex(s).unwrap();
    dbg!(&token_list);
    let (_, node) = parse_expression_kind(Tokens::new(&token_list)).unwrap();

    node
  }

  #[test]
  fn prefix_not() {
    let source = "!true";
    assert_eq!(
      compile(source),
      ExpressionKind::UnaryOperator(UnaryOperatorNode {
        kind: UnaryOperatorKind::Not,
        value: Box::new(ExpressionKind::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "true".to_owned()
          }]
        }))
      })
    );
  }

  #[test]
  fn prefix_negative() {
    let source = "-true";
    assert_eq!(
      compile(source),
      ExpressionKind::UnaryOperator(UnaryOperatorNode {
        kind: UnaryOperatorKind::Negative,
        value: Box::new(ExpressionKind::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "true".to_owned()
          }]
        }))
      })
    );
  }

  #[test]
  fn prefix_negative_negative() {
    let source = "--true";
    assert_eq!(
      compile(source),
      ExpressionKind::UnaryOperator(UnaryOperatorNode {
        kind: UnaryOperatorKind::Negative,
        value: Box::new(ExpressionKind::UnaryOperator(UnaryOperatorNode {
          kind: UnaryOperatorKind::Negative,
          value: Box::new(ExpressionKind::Path(PathNode {
            ident_list: vec![IdentNode {
              raw: "true".to_owned()
            }]
          }))
        }))
      })
    );
  }

  #[test]
  fn prefix_negative_int() {
    let source = "-1";
    assert_eq!(
      compile(source),
      ExpressionKind::UnaryOperator(UnaryOperatorNode {
        kind: UnaryOperatorKind::Negative,
        value: Box::new(ExpressionKind::Literal(LiteralKind::Int(1)))
      })
    );
  }

  #[test]
  fn index() {
    let source = "foo[1]";
    assert_eq!(
      compile(source),
      ExpressionKind::Index(IndexNode {
        array: Box::new(ExpressionKind::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "foo".to_owned()
          }]
        })),
        index: Box::new(ExpressionKind::Literal(LiteralKind::Int(1)))
      })
    );
  }

  #[test]
  fn input_generic() {
    let source = "foo<bar>";
    assert_eq!(
      compile(source),
      ExpressionKind::Generic(ExpressionGenericNode {
        expression: Box::new(ExpressionKind::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "foo".to_owned()
          }]
        })),
        generic_list: vec![ExpressionGenericKind::Input(TypeKind::Path(
          ImmutablityKind::Yes,
          PathNode {
            ident_list: vec![IdentNode {
              raw: "bar".to_owned()
            }]
          }
        ))]
      })
    );
  }

  #[test]
  fn output_generic() {
    let source = "foo<output = bar>";
    assert_eq!(
      compile(source),
      ExpressionKind::Generic(ExpressionGenericNode {
        expression: Box::new(ExpressionKind::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "foo".to_owned()
          }]
        })),
        generic_list: vec![ExpressionGenericKind::Output(
          IdentNode {
            raw: "output".to_owned()
          },
          TypeKind::Path(
            ImmutablityKind::Yes,
            PathNode {
              ident_list: vec![IdentNode {
                raw: "bar".to_owned()
              }]
            }
          )
        )]
      })
    );
  }

  #[test]
  fn await_operrator() {
    let source = "foo.await";
    assert_eq!(
      compile(source),
      ExpressionKind::Await(Box::new(ExpressionKind::Path(PathNode {
        ident_list: vec![IdentNode {
          raw: "foo".to_owned()
        }]
      })))
    );
  }

  #[test]
  fn tuple() {
    let source = "foo()";
    assert_eq!(
      compile(source),
      ExpressionKind::Tuple(TupleNode {
        field: Some(Box::new(ExpressionKind::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "foo".to_owned()
          }]
        }))),
        argument_list: vec![],
      })
    );
  }

  #[test]
  fn tuple_argument() {
    let source = "foo(bar)";
    assert_eq!(
      compile(source),
      ExpressionKind::Tuple(TupleNode {
        field: Some(Box::new(ExpressionKind::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "foo".to_owned()
          }]
        }))),
        argument_list: vec![TupleArgumentNode {
          name: None,
          value: ExpressionKind::Path(PathNode {
            ident_list: vec![IdentNode {
              raw: "bar".to_owned()
            }]
          })
        }],
      })
    );
  }

  #[test]
  fn tuple_named_argument() {
    let source = "foo(bar = baz)";
    assert_eq!(
      compile(source),
      ExpressionKind::Tuple(TupleNode {
        field: Some(Box::new(ExpressionKind::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "foo".to_owned()
          }]
        }))),
        argument_list: vec![TupleArgumentNode {
          name: Some(IdentNode {
            raw: "bar".to_owned()
          }),
          value: ExpressionKind::Path(PathNode {
            ident_list: vec![IdentNode {
              raw: "baz".to_owned()
            }]
          })
        }],
      })
    );
  }

  #[test]
  fn tuple_await_operator() {
    let source = "foo().await";
    assert_eq!(
      compile(source),
      ExpressionKind::Await(Box::new(ExpressionKind::Tuple(TupleNode {
        field: Some(Box::new(ExpressionKind::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "foo".to_owned()
          }]
        }))),
        argument_list: vec![],
      })))
    );
  }

  #[test]
  fn try_operator() {
    let source = "foo?";
    assert_eq!(
      compile(source),
      ExpressionKind::Try(Box::new(ExpressionKind::Path(PathNode {
        ident_list: vec![IdentNode {
          raw: "foo".to_owned()
        }]
      })))
    );
  }

  #[test]
  fn tuple_try_operator() {
    let source = "foo()?";
    assert_eq!(
      compile(source),
      ExpressionKind::Try(Box::new(ExpressionKind::Tuple(TupleNode {
        field: Some(Box::new(ExpressionKind::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "foo".to_owned()
          }]
        }))),
        argument_list: vec![],
      })))
    );
  }

  #[test]
  fn field() {
    let source = "foo.bar";
    assert_eq!(
      compile(source),
      ExpressionKind::Field(ExpressionKindFieldNode {
        lhs: Box::new(ExpressionKind::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "foo".to_owned()
          }]
        })),
        rhs: Box::new(IdentNode {
          raw: "bar".to_owned()
        })
      })
    );
  }

  #[test]
  fn arithemtic_add() {
    let source = "foo + bar";
    assert_eq!(
      compile(source),
      ExpressionKind::InfixOperator(InfixOperatorNode {
        kind: InfixOperatorKind::Add,
        lhs: Box::new(ExpressionKind::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "foo".to_owned()
          }]
        })),
        rhs: Box::new(ExpressionKind::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "bar".to_owned()
          }]
        }))
      })
    );
  }

  #[test]
  fn arithemtic_add_mul() {
    let source = "foo + bar * baz";
    assert_eq!(
      compile(source),
      ExpressionKind::InfixOperator(InfixOperatorNode {
        kind: InfixOperatorKind::Add,
        lhs: Box::new(ExpressionKind::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "foo".to_owned()
          }]
        })),
        rhs: Box::new(ExpressionKind::InfixOperator(InfixOperatorNode {
          kind: InfixOperatorKind::Mul,
          lhs: Box::new(ExpressionKind::Path(PathNode {
            ident_list: vec![IdentNode {
              raw: "bar".to_owned()
            }]
          })),
          rhs: Box::new(ExpressionKind::Path(PathNode {
            ident_list: vec![IdentNode {
              raw: "baz".to_owned()
            }]
          }))
        }))
      })
    );
  }

  #[test]
  fn arithemtic_mul_add() {
    let source = "foo * bar + baz";
    assert_eq!(
      compile(source),
      ExpressionKind::InfixOperator(InfixOperatorNode {
        kind: InfixOperatorKind::Add,
        lhs: Box::new(ExpressionKind::InfixOperator(InfixOperatorNode {
          kind: InfixOperatorKind::Mul,
          lhs: Box::new(ExpressionKind::Path(PathNode {
            ident_list: vec![IdentNode {
              raw: "foo".to_owned()
            }]
          })),
          rhs: Box::new(ExpressionKind::Path(PathNode {
            ident_list: vec![IdentNode {
              raw: "bar".to_owned()
            }]
          }))
        })),
        rhs: Box::new(ExpressionKind::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "baz".to_owned()
          }]
        }))
      })
    );
  }

  #[test]
  fn arithemtic_mul_add_mul() {
    let source = "foo * bar + baz * bax";
    assert_eq!(
      compile(source),
      ExpressionKind::InfixOperator(InfixOperatorNode {
        kind: InfixOperatorKind::Add,
        lhs: Box::new(ExpressionKind::InfixOperator(InfixOperatorNode {
          kind: InfixOperatorKind::Mul,
          lhs: Box::new(ExpressionKind::Path(PathNode {
            ident_list: vec![IdentNode {
              raw: "foo".to_owned()
            }]
          })),
          rhs: Box::new(ExpressionKind::Path(PathNode {
            ident_list: vec![IdentNode {
              raw: "bar".to_owned()
            }]
          }))
        })),
        rhs: Box::new(ExpressionKind::InfixOperator(InfixOperatorNode {
          kind: InfixOperatorKind::Mul,
          lhs: Box::new(ExpressionKind::Path(PathNode {
            ident_list: vec![IdentNode {
              raw: "baz".to_owned()
            }]
          })),
          rhs: Box::new(ExpressionKind::Path(PathNode {
            ident_list: vec![IdentNode {
              raw: "bax".to_owned()
            }]
          }))
        }))
      })
    );
  }

  #[test]
  fn struct_add_struct() {
    let source = "foo(bar) + baz(bax)";
    assert_eq!(
      compile(source),
      ExpressionKind::InfixOperator(InfixOperatorNode {
        kind: InfixOperatorKind::Add,
        lhs: Box::new(ExpressionKind::Tuple(TupleNode {
          field: Some(Box::new(ExpressionKind::Path(PathNode {
            ident_list: vec![IdentNode {
              raw: "foo".to_owned()
            }]
          }))),
          argument_list: vec![TupleArgumentNode {
            name: None,
            value: ExpressionKind::Path(PathNode {
              ident_list: vec![IdentNode {
                raw: "bar".to_owned()
              }]
            })
          }]
        })),
        rhs: Box::new(ExpressionKind::Tuple(TupleNode {
          field: Some(Box::new(ExpressionKind::Path(PathNode {
            ident_list: vec![IdentNode {
              raw: "baz".to_owned()
            }]
          }))),
          argument_list: vec![TupleArgumentNode {
            name: None,
            value: ExpressionKind::Path(PathNode {
              ident_list: vec![IdentNode {
                raw: "bax".to_owned()
              }]
            })
          }]
        })),
      })
    )
  }

  #[test]
  fn field_add_field() {
    let source = "foo.bar + baz.bax";
    assert_eq!(
      compile(source),
      ExpressionKind::InfixOperator(InfixOperatorNode {
        kind: InfixOperatorKind::Add,
        lhs: Box::new(ExpressionKind::Field(ExpressionKindFieldNode {
          lhs: Box::new(ExpressionKind::Path(PathNode {
            ident_list: vec![IdentNode {
              raw: "foo".to_owned()
            }]
          })),
          rhs: Box::new(IdentNode {
            raw: "bar".to_owned()
          })
        })),
        rhs: Box::new(ExpressionKind::Field(ExpressionKindFieldNode {
          lhs: Box::new(ExpressionKind::Path(PathNode {
            ident_list: vec![IdentNode {
              raw: "baz".to_owned()
            }]
          })),
          rhs: Box::new(IdentNode {
            raw: "bax".to_owned()
          })
        })),
      })
    )
  }

  #[test]
  fn logical() {
    let source = "foo & bar";
    assert_eq!(
      compile(source),
      ExpressionKind::InfixOperator(InfixOperatorNode {
        kind: InfixOperatorKind::BitAnd,
        lhs: Box::new(ExpressionKind::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "foo".to_owned()
          }]
        })),
        rhs: Box::new(ExpressionKind::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "bar".to_owned()
          }]
        }))
      })
    );
  }

  #[test]
  fn comparison() {
    let source = "foo < bar";
    assert_eq!(
      compile(source),
      ExpressionKind::InfixOperator(InfixOperatorNode {
        kind: InfixOperatorKind::LessThan,
        lhs: Box::new(ExpressionKind::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "foo".to_owned()
          }]
        })),
        rhs: Box::new(ExpressionKind::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "bar".to_owned()
          }]
        }))
      })
    );
  }

  #[test]
  fn lazy_boolean() {
    let source = "foo && bar";
    assert_eq!(
      compile(source),
      ExpressionKind::InfixOperator(InfixOperatorNode {
        kind: InfixOperatorKind::And,
        lhs: Box::new(ExpressionKind::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "foo".to_owned()
          }]
        })),
        rhs: Box::new(ExpressionKind::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "bar".to_owned()
          }]
        }))
      })
    );
  }

  #[test]
  fn pipeline_chain() {
    let source = "foo |> bar";
    assert_eq!(
      compile(source),
      ExpressionKind::InfixOperator(InfixOperatorNode {
        kind: InfixOperatorKind::ChainArrow,
        lhs: Box::new(ExpressionKind::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "foo".to_owned()
          }]
        })),
        rhs: Box::new(ExpressionKind::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "bar".to_owned()
          }]
        }))
      })
    );
  }

  #[test]
  fn block() {
    let source = "{ }";
    assert_eq!(
      compile(source),
      ExpressionKind::Block(BlockNode {
        statement_list: vec![]
      }),
    );
  }

  #[test]
  fn block_in_block() {
    let source = "{ { } }";
    assert_eq!(
      compile(source),
      ExpressionKind::Block(BlockNode {
        statement_list: vec![StatementKind::ExpressionKind(ExpressionKind::Block(
          BlockNode {
            statement_list: vec![]
          }
        ))]
      }),
    );
  }

  #[test]
  fn closure() {
    let source = "|| 1";
    assert_eq!(
      compile(source),
      ExpressionKind::Closure(ClosureNode {
        argument_list: vec![],
        return_type: None,
        block: BlockNode {
          statement_list: vec![StatementKind::ExpressionKind(ExpressionKind::Literal(
            LiteralKind::Int(1)
          ))]
        }
      }),
    );
  }
}
