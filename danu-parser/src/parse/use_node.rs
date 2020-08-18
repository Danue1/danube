use super::*;

pub(super) fn parse_use_node(s: Tokens) -> ParseResult<UseNode> {
  map(
    tuple((
      parse_keyword(Keyword::Use),
      parse_use_kind(parse_use_root_node),
      parse_symbol(Symbol::Semicolon),
    )),
    |(_, kind, _)| UseNode { kind },
  )(s)
}

#[cfg(test)]
mod tests {
  use super::*;

  fn compile(s: &str) -> UseNode {
    let (_, token_list) = lex(s).unwrap();
    let (_, node) = parse_use_node(Tokens::new(&token_list)).unwrap();

    node
  }

  #[test]
  fn self_unnested() {
    let source = "use self::foo;";
    assert_eq!(
      compile(source),
      UseNode {
        kind: UseKind::Unnested(UseRootNode {
          ident: UseRootIdent::Current,
          extra: UseKind::Unnested(UseExtra::Ident(
            IdentNode {
              raw: "foo".to_owned()
            },
            None
          ))
        })
      }
    );
  }

  #[test]
  fn super_unnested() {
    let source = "use super::foo;";
    assert_eq!(
      compile(source),
      UseNode {
        kind: UseKind::Unnested(UseRootNode {
          ident: UseRootIdent::Super,
          extra: UseKind::Unnested(UseExtra::Ident(
            IdentNode {
              raw: "foo".to_owned()
            },
            None
          ))
        })
      }
    );
  }

  #[test]
  fn module_unnested() {
    let source = "use mod::foo;";
    assert_eq!(
      compile(source),
      UseNode {
        kind: UseKind::Unnested(UseRootNode {
          ident: UseRootIdent::Module,
          extra: UseKind::Unnested(UseExtra::Ident(
            IdentNode {
              raw: "foo".to_owned()
            },
            None
          ))
        })
      }
    );
  }

  #[test]
  fn ident_unnested() {
    let source = "use foo::bar;";
    assert_eq!(
      compile(source),
      UseNode {
        kind: UseKind::Unnested(UseRootNode {
          ident: UseRootIdent::Ident(IdentNode {
            raw: "foo".to_owned()
          }),
          extra: UseKind::Unnested(UseExtra::Ident(
            IdentNode {
              raw: "bar".to_owned()
            },
            None
          ))
        })
      }
    );
  }

  #[test]
  fn ident_unnested_unnested() {
    let source = "use foo::bar::baz;";
    assert_eq!(
      compile(source),
      UseNode {
        kind: UseKind::Unnested(UseRootNode {
          ident: UseRootIdent::Ident(IdentNode {
            raw: "foo".to_owned()
          }),
          extra: UseKind::Unnested(UseExtra::Extra(
            IdentNode {
              raw: "bar".to_owned()
            },
            Box::new(UseKind::Unnested(UseExtra::Ident(
              IdentNode {
                raw: "baz".to_owned()
              },
              None
            )))
          ))
        })
      }
    );
  }

  #[test]
  fn self_nested_single() {
    let source = "use {self::bar};";
    assert_eq!(
      compile(source),
      UseNode {
        kind: UseKind::Nested(vec![UseRootNode {
          ident: UseRootIdent::Current,
          extra: UseKind::Unnested(UseExtra::Ident(
            IdentNode {
              raw: "bar".to_owned()
            },
            None
          ))
        }])
      }
    );
  }

  #[test]
  fn self_nested_double() {
    let source = "use {self::bar, self::baz};";
    assert_eq!(
      compile(source),
      UseNode {
        kind: UseKind::Nested(vec![
          UseRootNode {
            ident: UseRootIdent::Current,
            extra: UseKind::Unnested(UseExtra::Ident(
              IdentNode {
                raw: "bar".to_owned()
              },
              None
            ))
          },
          UseRootNode {
            ident: UseRootIdent::Current,
            extra: UseKind::Unnested(UseExtra::Ident(
              IdentNode {
                raw: "baz".to_owned()
              },
              None
            ))
          }
        ])
      }
    );
  }

  #[test]
  fn self_nested_alias() {
    let source = "use {self::bar as baz};";
    assert_eq!(
      compile(source),
      UseNode {
        kind: UseKind::Nested(vec![UseRootNode {
          ident: UseRootIdent::Current,
          extra: UseKind::Unnested(UseExtra::Ident(
            IdentNode {
              raw: "bar".to_owned()
            },
            Some(IdentNode {
              raw: "baz".to_owned()
            })
          ))
        }])
      }
    );
  }
}
