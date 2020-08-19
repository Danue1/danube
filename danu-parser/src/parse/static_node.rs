use super::*;

pub(super) fn parse_static_node(s: Tokens) -> ParseResult<StaticNode> {
  map(
    tuple((
      opt(parse_visibility),
      parse_keyword(Keyword::Static),
      parse_ident_node,
      parse_symbol(Symbol::Colon),
      parse_type_node,
      parse_symbol(Symbol::Assign),
      parse_literal_value_node,
      parse_symbol(Symbol::Semicolon),
    )),
    |(visibility, _, ident, _, ty, _, value, _)| StaticNode {
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

  fn compile(s: &str) -> StaticNode {
    let (_, token_list) = lex(s).unwrap();
    let (_, node) = parse_static_node(Tokens::new(&token_list)).unwrap();

    node
  }

  #[test]
  fn test() {
    let source = "static FOO: bool = true;";
    assert_eq!(
      compile(source),
      StaticNode {
        visibility: None,
        ident: IdentNode {
          raw: "FOO".to_owned()
        },
        ty: TypeNode::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "bool".to_owned()
          }]
        }),
        value: LiteralValueNode::Bool(true)
      }
    );
  }
}
