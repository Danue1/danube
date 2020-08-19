use super::*;

pub(super) fn parse_type_alias_node(s: Tokens) -> ParseResult<TypeAliasNode> {
  map(
    tuple((
      opt(parse_visibility),
      parse_keyword(Keyword::Type),
      parse_ident_node,
      parse_symbol(Symbol::Assign),
      parse_type_node,
      parse_symbol(Symbol::Semicolon),
    )),
    |(visibility, _, ident, _, ty, _)| TypeAliasNode {
      visibility,
      ident,
      ty,
    },
  )(s)
}

#[cfg(test)]
mod tests {
  use super::*;

  fn compile(s: &str) -> TypeAliasNode {
    let (_, token_list) = lex(s).unwrap();
    let (_, node) = parse_type_alias_node(Tokens::new(&token_list)).unwrap();

    node
  }

  #[test]
  fn test() {
    let source = "type Foo = bool;";
    assert_eq!(
      compile(source),
      TypeAliasNode {
        visibility: None,
        ident: IdentNode {
          raw: "Foo".to_owned()
        },
        ty: TypeNode::Path(
          TypeImmutablity::Yes,
          PathNode {
            ident_list: vec![IdentNode {
              raw: "bool".to_owned()
            }]
          }
        )
      }
    );
  }
}
