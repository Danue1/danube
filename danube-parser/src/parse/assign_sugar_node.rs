use super::*;

pub(super) fn parse_assign_sugar_node(s: Tokens) -> ParseResult<AssignSugarNode> {
  map(
    tuple((
      parse_ident_node,
      parse_assign_sugar_kind,
      parse_expression_node,
      parse_symbol(Symbol::Semicolon),
    )),
    |(ident, kind, value, _)| AssignSugarNode { ident, kind, value },
  )(s)
}

#[cfg(test)]
mod tests {
  use super::*;

  fn compile(s: &str) -> AssignSugarNode {
    let (_, token_list) = lex(s).unwrap();
    let (_, node) = parse_assign_sugar_node(Tokens::new(&token_list)).unwrap();

    node
  }

  #[test]
  fn test() {
    let source = "foo += 1;";
    assert_eq!(
      compile(source),
      AssignSugarNode {
        kind: AssignSugarKind::AddAssign,
        ident: IdentNode {
          raw: "foo".to_owned()
        },
        value: ExpressionNode::Literal(LiteralValueNode::Int(1))
      }
    );
  }
}
