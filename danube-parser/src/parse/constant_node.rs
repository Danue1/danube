use super::*;

pub(super) fn parse_constant_node(s: Tokens) -> ParseResult<ConstantNode> {
  map(
    tuple((
      opt(parse_visibility_kind),
      parse_keyword(Keyword::Const),
      parse_ident_node,
      parse_symbol(Symbol::Colon),
      parse_type_kind,
      parse_symbol(Symbol::Assign),
      parse_expression_kind,
      parse_symbol(Symbol::Semicolon),
    )),
    |(visibility, _, ident, _, ty, _, value, _)| ConstantNode {
      visibility,
      ident,
      ty,
      value,
    },
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
        visibility: None,
        ident: IdentNode {
          raw: "FOO".to_owned()
        },
        ty: TypeKind::Path(
          ImmutablityKind::Yes,
          PathNode {
            ident_list: vec![IdentNode {
              raw: "bool".to_owned()
            }]
          }
        ),
        value: ExpressionKind::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "true".to_owned()
          }]
        })
      }
    );
  }

  #[test]
  fn array_type() {
    let source = "const FOO: [bool; 0] = true;";
    assert_eq!(
      compile(source),
      ConstantNode {
        visibility: None,
        ident: IdentNode {
          raw: "FOO".to_owned()
        },
        ty: TypeKind::Array(
          ImmutablityKind::Yes,
          TypeArrayNode {
            ty: Box::new(TypeKind::Path(
              ImmutablityKind::Yes,
              PathNode {
                ident_list: vec![IdentNode {
                  raw: "bool".to_owned()
                }]
              }
            )),
            size: 0
          }
        ),
        value: ExpressionKind::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "true".to_owned()
          }]
        })
      }
    );
  }

  #[test]
  fn array_value_empty() {
    let source = "const FOO: bool = [];";
    assert_eq!(
      compile(source),
      ConstantNode {
        visibility: None,
        ident: IdentNode {
          raw: "FOO".to_owned()
        },
        ty: TypeKind::Path(
          ImmutablityKind::Yes,
          PathNode {
            ident_list: vec![IdentNode {
              raw: "bool".to_owned()
            }]
          }
        ),
        value: ExpressionKind::Array(vec![])
      }
    );
  }

  #[test]
  fn array_value_a_element() {
    let source = "const FOO: bool = [true];";
    assert_eq!(
      compile(source),
      ConstantNode {
        visibility: None,
        ident: IdentNode {
          raw: "FOO".to_owned()
        },
        ty: TypeKind::Path(
          ImmutablityKind::Yes,
          PathNode {
            ident_list: vec![IdentNode {
              raw: "bool".to_owned()
            }]
          }
        ),
        value: ExpressionKind::Array(vec![ExpressionKind::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "true".to_owned()
          }]
        })])
      }
    );
  }
}
