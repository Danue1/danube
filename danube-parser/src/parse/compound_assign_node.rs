use super::*;

pub(super) fn parse_compound_assign_node(s: Tokens) -> ParseResult<CompoundAssignNode> {
  map(
    tuple((
      parse_expression_node,
      parse_compound_assign_kind,
      parse_expression_node,
      parse_symbol(Symbol::Semicolon),
    )),
    |(lhs, kind, rhs, _)| CompoundAssignNode { kind, lhs, rhs },
  )(s)
}

#[cfg(test)]
mod tests {
  use super::*;

  fn compile(s: &str) -> CompoundAssignNode {
    let (_, token_list) = lex(s).unwrap();
    let (_, node) = parse_compound_assign_node(Tokens::new(&token_list)).unwrap();

    node
  }

  #[test]
  fn variable() {
    let source = "foo += 1;";
    assert_eq!(
      compile(source),
      CompoundAssignNode {
        kind: CompoundAssignKind::AddAssign,
        lhs: ExpressionNode::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "foo".to_owned(),
          }]
        }),
        rhs: ExpressionNode::Literal(LiteralValueNode::Int(1))
      }
    );
  }

  #[test]
  fn variable_field() {
    let source = "foo.bar += 1;";
    assert_eq!(
      compile(source),
      CompoundAssignNode {
        kind: CompoundAssignKind::AddAssign,
        lhs: ExpressionNode::Field(ExpressionFieldNode {
          lhs: Box::new(ExpressionNode::Path(PathNode {
            ident_list: vec![IdentNode {
              raw: "foo".to_owned(),
            }]
          })),
          rhs: Box::new(IdentNode {
            raw: "bar".to_owned()
          }),
        }),
        rhs: ExpressionNode::Literal(LiteralValueNode::Int(1))
      }
    );
  }
}
