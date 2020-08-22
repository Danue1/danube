use super::*;

pub(super) fn parse_expression_node(s: Tokens) -> ParseResult<ExpressionNode> {
  let (s, node) = parse_prefixable_expression_node(s)?;

  parse_postfix_expression_node(s, Precedence::Lowest, node)
}

fn parse_prefixable_expression_node(s: Tokens) -> ParseResult<ExpressionNode> {
  alt((parse_prefix_expression_node, parse_atomic_expression_node))(s)
}

fn parse_atomic_expression_node(s: Tokens) -> ParseResult<ExpressionNode> {
  alt((
    map(parse_expression_struct_node, ExpressionNode::Struct),
    map(parse_expression_tuple_node, ExpressionNode::Tuple),
    map(parse_path_node, ExpressionNode::Path),
    map(
      parse_expression_conditional_node,
      ExpressionNode::Conditional,
    ),
    map(parse_loop_node, ExpressionNode::Loop),
    map(parse_while_node, ExpressionNode::While),
    map(parse_for_node, ExpressionNode::For),
    map(parse_pattern_match_node, ExpressionNode::PatternMatch),
    map(parse_literal_value_node, ExpressionNode::Literal),
    map(parse_break, |_| ExpressionNode::Break),
    map(parse_continue, |_| ExpressionNode::Continue),
    map(parse_return_node, ExpressionNode::Return),
    map(parse_array, ExpressionNode::Array),
  ))(s)
}

fn parse_expression_struct_node(s: Tokens) -> ParseResult<ExpressionStructNode> {
  map(parse_expression_field_list, |(field_list, rest)| {
    ExpressionStructNode {
      path: None,
      field_list,
      rest,
    }
  })(s)
}

fn parse_expression_tuple_node(s: Tokens) -> ParseResult<TupleNode> {
  map(parse_tuple_operator, |node_list| TupleNode {
    field: None,
    node_list,
  })(s)
}

fn parse_prefix_expression_node(s: Tokens) -> ParseResult<ExpressionNode> {
  map(
    tuple((parse_unary_operator_kind, parse_expression_node)),
    |(kind, value)| {
      ExpressionNode::UnaryOperator(UnaryOperatorNode {
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
  lhs: ExpressionNode,
) -> ParseResult<ExpressionNode> {
  match parse_operator_kind(s.clone()) {
    Ok((s, OperatorKind::Await)) => {
      let node = ExpressionNode::Await(Box::new(lhs));

      parse_postfix_expression_node(s, precedence, node)
    }
    Ok((s, OperatorKind::Try)) => {
      let node = ExpressionNode::Try(Box::new(lhs));

      parse_postfix_expression_node(s, precedence, node)
    }
    Ok((s, OperatorKind::Tuple(node_list))) => {
      let node = ExpressionNode::Tuple(TupleNode {
        field: Some(Box::new(lhs)),
        node_list,
      });

      parse_postfix_expression_node(s, precedence, node)
    }
    Ok((s, OperatorKind::Index(rhs))) => {
      let node = ExpressionNode::Index(IndexNode {
        array: Box::new(lhs),
        index: Box::new(rhs),
      });

      parse_postfix_expression_node(s, precedence, node)
    }
    Ok((s, OperatorKind::Field(rhs))) => {
      let node = ExpressionNode::Field(ExpressionFieldNode {
        left: Box::new(lhs),
        right: Box::new(rhs),
      });

      parse_postfix_expression_node(s, precedence, node)
    }
    Ok((_, OperatorKind::InfixOperator(_))) => parse_infix_expression_node(s, precedence, lhs),
    _ => {
      if let ExpressionNode::Path(path) = lhs.clone() {
        if let Ok((s, (field_list, rest))) = parse_expression_field_list(s.clone()) {
          let node = ExpressionNode::Struct(ExpressionStructNode {
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
  lhs: ExpressionNode,
) -> ParseResult<ExpressionNode> {
  if let Ok((ss, kind)) = parse_infix_operator_kind(s.clone()) {
    let right_precedence = infix_binding_power(kind.clone());
    if right_precedence < precedence {
      Ok((s, lhs))
    } else {
      let (s, rhs) = parse_prefixable_expression_node(ss)?;
      let (s, rhs) = parse_postfix_expression_node(s, right_precedence, rhs)?;
      let rhs = ExpressionNode::InfixOperator(InfixOperatorNode {
        kind,
        left: Box::new(lhs),
        right: Box::new(rhs),
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
  Tuple(Vec<ExpressionNode>),
  Index(ExpressionNode),
  Field(IdentNode),
  InfixOperator(InfixOperatorKind),
}

fn parse_operator_kind(s: Tokens) -> ParseResult<OperatorKind> {
  alt((
    map(parse_await_operator, |_| OperatorKind::Await),
    map(parse_try_operator, |_| OperatorKind::Try),
    map(parse_tuple_operator, OperatorKind::Tuple),
    map(parse_index_operator, OperatorKind::Index),
    map(parse_field_operator, OperatorKind::Field),
    map(parse_infix_operator_kind, OperatorKind::InfixOperator),
  ))(s)
}

fn parse_tuple_operator(s: Tokens) -> ParseResult<Vec<ExpressionNode>> {
  map(
    tuple((
      parse_symbol(Symbol::LeftParens),
      separated_list(parse_symbol(Symbol::Comma), parse_expression_node),
      opt(parse_symbol(Symbol::Comma)),
      parse_symbol(Symbol::RightParens),
    )),
    |(_, expression_list, _, _)| expression_list,
  )(s)
}

fn parse_index_operator(s: Tokens) -> ParseResult<ExpressionNode> {
  map(
    tuple((
      parse_symbol(Symbol::LeftBracket),
      parse_expression_node,
      parse_symbol(Symbol::RightBracket),
    )),
    |(_, expression, _)| expression,
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
) -> ParseResult<(Vec<ExpressionStructField>, Option<Box<ExpressionNode>>)> {
  map(
    tuple((
      parse_symbol(Symbol::LeftBrace),
      separated_nonempty_list(
        parse_symbol(Symbol::Comma),
        tuple((
          parse_ident_node,
          opt(map(
            tuple((parse_symbol(Symbol::DoubleColon), parse_expression_node)),
            |(_, expression)| expression,
          )),
        )),
      ),
      opt(parse_symbol(Symbol::Comma)),
      opt(map(
        tuple((parse_symbol(Symbol::RangeClose), parse_expression_node)),
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

fn parse_array(s: Tokens) -> ParseResult<Vec<ExpressionNode>> {
  map(
    tuple((
      parse_symbol(Symbol::LeftBracket),
      separated_list(parse_symbol(Symbol::Comma), parse_expression_node),
      opt(parse_symbol(Symbol::Comma)),
      parse_symbol(Symbol::RightBracket),
    )),
    |(_, expression_list, _, _)| expression_list,
  )(s)
}

#[cfg(test)]
mod tests {
  use super::*;

  fn compile(s: &str) -> ExpressionNode {
    let (_, token_list) = lex(s).unwrap();
    dbg!(&token_list);
    let (_, node) = parse_expression_node(Tokens::new(&token_list)).unwrap();

    node
  }

  #[test]
  fn prefix_not() {
    let source = "!true";
    assert_eq!(
      compile(source),
      ExpressionNode::UnaryOperator(UnaryOperatorNode {
        kind: UnaryOperatorKind::Not,
        value: Box::new(ExpressionNode::Literal(LiteralValueNode::Bool(true)))
      })
    );
  }

  #[test]
  fn prefix_negative() {
    let source = "-true";
    assert_eq!(
      compile(source),
      ExpressionNode::UnaryOperator(UnaryOperatorNode {
        kind: UnaryOperatorKind::Negative,
        value: Box::new(ExpressionNode::Literal(LiteralValueNode::Bool(true)))
      })
    );
  }

  #[test]
  fn prefix_negative_negative() {
    let source = "--true";
    assert_eq!(
      compile(source),
      ExpressionNode::UnaryOperator(UnaryOperatorNode {
        kind: UnaryOperatorKind::Negative,
        value: Box::new(ExpressionNode::UnaryOperator(UnaryOperatorNode {
          kind: UnaryOperatorKind::Negative,
          value: Box::new(ExpressionNode::Literal(LiteralValueNode::Bool(true)))
        }))
      })
    );
  }

  #[test]
  fn prefix_negative_int() {
    let source = "-1";
    assert_eq!(
      compile(source),
      ExpressionNode::UnaryOperator(UnaryOperatorNode {
        kind: UnaryOperatorKind::Negative,
        value: Box::new(ExpressionNode::Literal(LiteralValueNode::Int(1)))
      })
    );
  }

  #[test]
  fn index() {
    let source = "foo[1]";
    assert_eq!(
      compile(source),
      ExpressionNode::Index(IndexNode {
        array: Box::new(ExpressionNode::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "foo".to_owned()
          }]
        })),
        index: Box::new(ExpressionNode::Literal(LiteralValueNode::Int(1)))
      })
    );
  }

  #[test]
  fn await_operrator() {
    let source = "foo.await";
    assert_eq!(
      compile(source),
      ExpressionNode::Await(Box::new(ExpressionNode::Path(PathNode {
        ident_list: vec![IdentNode {
          raw: "foo".to_owned()
        }]
      })))
    );
  }

  #[test]
  fn tuple_await_operator() {
    let source = "foo().await";
    assert_eq!(
      compile(source),
      ExpressionNode::Await(Box::new(ExpressionNode::Tuple(TupleNode {
        field: Some(Box::new(ExpressionNode::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "foo".to_owned()
          }]
        }))),
        node_list: vec![],
      })))
    );
  }

  #[test]
  fn try_operator() {
    let source = "foo?";
    assert_eq!(
      compile(source),
      ExpressionNode::Try(Box::new(ExpressionNode::Path(PathNode {
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
      ExpressionNode::Try(Box::new(ExpressionNode::Tuple(TupleNode {
        field: Some(Box::new(ExpressionNode::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "foo".to_owned()
          }]
        }))),
        node_list: vec![],
      })))
    );
  }

  #[test]
  fn field() {
    let source = "foo.bar";
    assert_eq!(
      compile(source),
      ExpressionNode::Field(ExpressionFieldNode {
        left: Box::new(ExpressionNode::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "foo".to_owned()
          }]
        })),
        right: Box::new(IdentNode {
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
      ExpressionNode::InfixOperator(InfixOperatorNode {
        kind: InfixOperatorKind::Add,
        left: Box::new(ExpressionNode::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "foo".to_owned()
          }]
        })),
        right: Box::new(ExpressionNode::Path(PathNode {
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
      ExpressionNode::InfixOperator(InfixOperatorNode {
        kind: InfixOperatorKind::Add,
        left: Box::new(ExpressionNode::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "foo".to_owned()
          }]
        })),
        right: Box::new(ExpressionNode::InfixOperator(InfixOperatorNode {
          kind: InfixOperatorKind::Mul,
          left: Box::new(ExpressionNode::Path(PathNode {
            ident_list: vec![IdentNode {
              raw: "bar".to_owned()
            }]
          })),
          right: Box::new(ExpressionNode::Path(PathNode {
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
      ExpressionNode::InfixOperator(InfixOperatorNode {
        kind: InfixOperatorKind::Add,
        left: Box::new(ExpressionNode::InfixOperator(InfixOperatorNode {
          kind: InfixOperatorKind::Mul,
          left: Box::new(ExpressionNode::Path(PathNode {
            ident_list: vec![IdentNode {
              raw: "foo".to_owned()
            }]
          })),
          right: Box::new(ExpressionNode::Path(PathNode {
            ident_list: vec![IdentNode {
              raw: "bar".to_owned()
            }]
          }))
        })),
        right: Box::new(ExpressionNode::Path(PathNode {
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
      ExpressionNode::InfixOperator(InfixOperatorNode {
        kind: InfixOperatorKind::Add,
        left: Box::new(ExpressionNode::InfixOperator(InfixOperatorNode {
          kind: InfixOperatorKind::Mul,
          left: Box::new(ExpressionNode::Path(PathNode {
            ident_list: vec![IdentNode {
              raw: "foo".to_owned()
            }]
          })),
          right: Box::new(ExpressionNode::Path(PathNode {
            ident_list: vec![IdentNode {
              raw: "bar".to_owned()
            }]
          }))
        })),
        right: Box::new(ExpressionNode::InfixOperator(InfixOperatorNode {
          kind: InfixOperatorKind::Mul,
          left: Box::new(ExpressionNode::Path(PathNode {
            ident_list: vec![IdentNode {
              raw: "baz".to_owned()
            }]
          })),
          right: Box::new(ExpressionNode::Path(PathNode {
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
      ExpressionNode::InfixOperator(InfixOperatorNode {
        kind: InfixOperatorKind::Add,
        left: Box::new(ExpressionNode::Tuple(TupleNode {
          field: Some(Box::new(ExpressionNode::Path(PathNode {
            ident_list: vec![IdentNode {
              raw: "foo".to_owned()
            }]
          }))),
          node_list: vec![ExpressionNode::Path(PathNode {
            ident_list: vec![IdentNode {
              raw: "bar".to_owned()
            }]
          })]
        })),
        right: Box::new(ExpressionNode::Tuple(TupleNode {
          field: Some(Box::new(ExpressionNode::Path(PathNode {
            ident_list: vec![IdentNode {
              raw: "baz".to_owned()
            }]
          }))),
          node_list: vec![ExpressionNode::Path(PathNode {
            ident_list: vec![IdentNode {
              raw: "bax".to_owned()
            }]
          })]
        })),
      })
    )
  }

  #[test]
  fn field_add_field() {
    let source = "foo.bar + baz.bax";
    assert_eq!(
      compile(source),
      ExpressionNode::InfixOperator(InfixOperatorNode {
        kind: InfixOperatorKind::Add,
        left: Box::new(ExpressionNode::Field(ExpressionFieldNode {
          left: Box::new(ExpressionNode::Path(PathNode {
            ident_list: vec![IdentNode {
              raw: "foo".to_owned()
            }]
          })),
          right: Box::new(IdentNode {
            raw: "bar".to_owned()
          })
        })),
        right: Box::new(ExpressionNode::Field(ExpressionFieldNode {
          left: Box::new(ExpressionNode::Path(PathNode {
            ident_list: vec![IdentNode {
              raw: "baz".to_owned()
            }]
          })),
          right: Box::new(IdentNode {
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
      ExpressionNode::InfixOperator(InfixOperatorNode {
        kind: InfixOperatorKind::BitAnd,
        left: Box::new(ExpressionNode::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "foo".to_owned()
          }]
        })),
        right: Box::new(ExpressionNode::Path(PathNode {
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
      ExpressionNode::InfixOperator(InfixOperatorNode {
        kind: InfixOperatorKind::LessThan,
        left: Box::new(ExpressionNode::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "foo".to_owned()
          }]
        })),
        right: Box::new(ExpressionNode::Path(PathNode {
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
      ExpressionNode::InfixOperator(InfixOperatorNode {
        kind: InfixOperatorKind::And,
        left: Box::new(ExpressionNode::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "foo".to_owned()
          }]
        })),
        right: Box::new(ExpressionNode::Path(PathNode {
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
      ExpressionNode::InfixOperator(InfixOperatorNode {
        kind: InfixOperatorKind::ChainArrow,
        left: Box::new(ExpressionNode::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "foo".to_owned()
          }]
        })),
        right: Box::new(ExpressionNode::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "bar".to_owned()
          }]
        }))
      })
    );
  }
}
