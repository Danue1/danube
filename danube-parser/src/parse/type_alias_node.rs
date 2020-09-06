use super::*;

pub(super) fn parse_type_alias_node(s: Tokens) -> ParseResult<TypeAliasNode> {
  map(
    tuple((
      opt(parse_visibility_kind),
      parse_keyword(Keyword::Type),
      parse_ident_node,
      parse_symbol(Symbol::Assign),
      parse_type_kind,
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
  fn path() {
    let source = "type Foo = bool;";
    assert_eq!(
      compile(source),
      TypeAliasNode {
        visibility: None,
        ident: IdentNode {
          raw: "Foo".to_owned()
        },
        ty: TypeKind::Path(
          ImmutablityKind::Yes,
          PathNode {
            ident_list: vec![IdentNode {
              raw: "bool".to_owned()
            }]
          }
        )
      }
    );
  }

  #[test]
  fn tuple() {
    let source = "type Foo = ();";
    assert_eq!(
      compile(source),
      TypeAliasNode {
        visibility: None,
        ident: IdentNode {
          raw: "Foo".to_owned()
        },
        ty: TypeKind::Tuple(ImmutablityKind::Yes, vec![])
      }
    );
  }
}
