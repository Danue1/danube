use super::*;

pub(super) fn parse_constant_node(s: Tokens) -> ParseResult<ConstantNode> {
  map(
    tuple((
      parse_keyword(Keyword::Const),
      parse_ident_node,
      parse_symbol(Symbol::Colon),
      parse_type_node,
      parse_symbol(Symbol::Assign),
      parse_expression_node,
      parse_symbol(Symbol::Semicolon),
    )),
    |(_, ident, _, ty, _, value, _)| ConstantNode { ident, ty, value },
  )(s)
}

#[cfg(test)]
mod tests {
  use super::*;

  fn compile(s: &str) -> ConstantNode {
    let (_, token_list) = lex(s).unwrap();
    let (_, node) = parse_constant_node(Tokens::new(&token_list)).unwrap();

    node
  }

  #[test]
  fn test() {
    let source = "const FOO: bool = true;";
    assert_eq!(
      compile(source),
      ConstantNode {
        ident: IdentNode {
          raw: "FOO".to_owned()
        },
        ty: TypeNode::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "bool".to_owned()
          }]
        }),
        value: ExpressionNode::Literal(LiteralValueNode::Bool(true))
      }
    );
  }

  #[test]
  fn array_type() {
    let source = "const FOO: [bool; 0] = true;";
    assert_eq!(
      compile(source),
      ConstantNode {
        ident: IdentNode {
          raw: "FOO".to_owned()
        },
        ty: TypeNode::Array(Box::new(TypeArrayNode {
          ty: TypeNode::Path(PathNode {
            ident_list: vec![IdentNode {
              raw: "bool".to_owned()
            }]
          }),
          size: 0
        })),
        value: ExpressionNode::Literal(LiteralValueNode::Bool(true))
      }
    );
  }

  #[test]
  fn array_value_empty() {
    let source = "const FOO: bool = [];";
    assert_eq!(
      compile(source),
      ConstantNode {
        ident: IdentNode {
          raw: "FOO".to_owned()
        },
        ty: TypeNode::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "bool".to_owned()
          }]
        }),
        value: ExpressionNode::Array(vec![])
      }
    );
  }

  #[test]
  fn array_value_a_element() {
    let source = "const FOO: bool = [true];";
    assert_eq!(
      compile(source),
      ConstantNode {
        ident: IdentNode {
          raw: "FOO".to_owned()
        },
        ty: TypeNode::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "bool".to_owned()
          }]
        }),
        value: ExpressionNode::Array(vec![ExpressionNode::Literal(LiteralValueNode::Bool(true))])
      }
    );
  }
}
