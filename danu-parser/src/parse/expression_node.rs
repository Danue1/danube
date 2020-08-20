use super::*;

pub(super) fn parse_expression_node(s: Tokens) -> ParseResult<ExpressionNode> {
  let (s, node) = alt((parse_prefix_expression_node, parse_atomic_expression_node))(s)?;

  parse_postfix_expression_node(s, node)
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
    tuple((parse_unary_operator_kind, parse_atomic_expression_node)),
    |(kind, value)| {
      ExpressionNode::UnaryOperator(UnaryOperatorNode {
        kind,
        value: Box::new(value),
      })
    },
  )(s)
}

fn parse_postfix_expression_node(s: Tokens, left: ExpressionNode) -> ParseResult<ExpressionNode> {
  match parse_operator_kind(s.clone()) {
    Ok((s, OperatorKind::Await)) => {
      let node = ExpressionNode::Await(Box::new(left));

      parse_postfix_expression_node(s, node)
    }
    Ok((s, OperatorKind::Try)) => {
      let node = ExpressionNode::Try(Box::new(left));

      parse_postfix_expression_node(s, node)
    }
    Ok((s, OperatorKind::Tuple(node_list))) => {
      let node = ExpressionNode::Tuple(TupleNode {
        field: Some(Box::new(left)),
        node_list,
      });

      parse_postfix_expression_node(s, node)
    }
    Ok((s, OperatorKind::Index(right))) => {
      let node = ExpressionNode::Index(IndexNode {
        array: Box::new(left),
        index: Box::new(right),
      });

      parse_postfix_expression_node(s, node)
    }
    Ok((s, OperatorKind::Field(right))) => {
      let node = ExpressionNode::Field(ExpressionFieldNode {
        left: Box::new(left),
        right: Box::new(right),
      });

      parse_postfix_expression_node(s, node)
    }
    Ok((s, OperatorKind::PipelineChain)) => {
      let (s, right) = parse_expression_node(s)?;
      let node = ExpressionNode::PipelineChain(PipelineChainNode {
        left: Box::new(left),
        right: Box::new(right),
      });

      parse_postfix_expression_node(s, node)
    }
    Ok((s, OperatorKind::ArithmeticOrLogical(kind))) => {
      let (s, right) = parse_expression_node(s)?;
      let node = ExpressionNode::ArithmeticOrLogical(ArithmeticOrLogicalNode {
        kind,
        left: Box::new(left),
        right: Box::new(right),
      });

      parse_postfix_expression_node(s, node)
    }
    Ok((s, OperatorKind::Comparison(kind))) => {
      let (s, right) = parse_expression_node(s)?;
      let node = ExpressionNode::Comparison(ComparisonNode {
        kind,
        left: Box::new(left),
        right: Box::new(right),
      });

      parse_postfix_expression_node(s, node)
    }
    Ok((s, OperatorKind::LazyBoolean(kind))) => {
      let (s, right) = parse_expression_node(s)?;
      let node = ExpressionNode::LazyBooleann(LazyBooleanNode {
        kind,
        left: Box::new(left),
        right: Box::new(right),
      });

      parse_postfix_expression_node(s, node)
    }
    _ => {
      if let ExpressionNode::Path(path) = left.clone() {
        if let Ok((s, (field_list, rest))) = parse_expression_field_list(s.clone()) {
          let node = ExpressionNode::Struct(ExpressionStructNode {
            path: Some(path),
            field_list,
            rest,
          });

          parse_postfix_expression_node(s, node)
        } else {
          Ok((s, left))
        }
      } else {
        Ok((s, left))
      }
    }
  }
}

enum OperatorKind {
  Await,
  Try,
  Tuple(Vec<ExpressionNode>),
  Index(ExpressionNode),
  Field(IdentNode),
  PipelineChain,
  ArithmeticOrLogical(ArithmeticOrLogicalKind),
  Comparison(ComparisonKind),
  LazyBoolean(LazyBooleanKind),
}

fn parse_operator_kind(s: Tokens) -> ParseResult<OperatorKind> {
  alt((
    map(parse_await_operator, |_| OperatorKind::Await),
    map(parse_try_operator, |_| OperatorKind::Try),
    map(parse_tuple_operator, OperatorKind::Tuple),
    map(parse_index_operator, OperatorKind::Index),
    map(parse_field_operator, OperatorKind::Field),
    map(parse_pipeline_chain_operator, |_| {
      OperatorKind::PipelineChain
    }),
    map(
      parse_arithmetic_or_logical_kind,
      OperatorKind::ArithmeticOrLogical,
    ),
    map(parse_comparison_kind, OperatorKind::Comparison),
    map(parse_lazy_boolean_kind, OperatorKind::LazyBoolean),
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

fn parse_pipeline_chain_operator(s: Tokens) -> ParseResult<()> {
  map(parse_symbol(Symbol::ChainArrow), |_| ())(s)
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
      ExpressionNode::ArithmeticOrLogical(ArithmeticOrLogicalNode {
        kind: ArithmeticOrLogicalKind::Add,
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
      ExpressionNode::ArithmeticOrLogical(ArithmeticOrLogicalNode {
        kind: ArithmeticOrLogicalKind::Add,
        left: Box::new(ExpressionNode::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "foo".to_owned()
          }]
        })),
        right: Box::new(ExpressionNode::ArithmeticOrLogical(
          ArithmeticOrLogicalNode {
            kind: ArithmeticOrLogicalKind::Mul,
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
          }
        ))
      })
    );
  }

  #[test]
  #[ignore]
  fn arithemtic_mul_add() {
    let source = "foo * bar + baz";
    assert_eq!(
      compile(source),
      ExpressionNode::ArithmeticOrLogical(ArithmeticOrLogicalNode {
        kind: ArithmeticOrLogicalKind::Add,
        left: Box::new(ExpressionNode::ArithmeticOrLogical(
          ArithmeticOrLogicalNode {
            kind: ArithmeticOrLogicalKind::Mul,
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
          }
        )),
        right: Box::new(ExpressionNode::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "baz".to_owned()
          }]
        }))
      })
    );
  }

  #[test]
  fn logical() {
    let source = "foo & bar";
    assert_eq!(
      compile(source),
      ExpressionNode::ArithmeticOrLogical(ArithmeticOrLogicalNode {
        kind: ArithmeticOrLogicalKind::BitAnd,
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
      ExpressionNode::Comparison(ComparisonNode {
        kind: ComparisonKind::LessThan,
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
      ExpressionNode::LazyBooleann(LazyBooleanNode {
        kind: LazyBooleanKind::And,
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
      ExpressionNode::PipelineChain(PipelineChainNode {
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
