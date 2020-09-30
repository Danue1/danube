use super::*;

pub(super) fn parse_implement_trait_node(s: Tokens) -> ParseResult<ImplementTraitNode> {
  map(
    tuple((
      opt(parse_visibility_kind),
      parse_keyword(Keyword::Impl),
      parse_path_node,
      opt(parse_generic_node),
      parse_keyword(Keyword::For),
      parse_path_node,
      opt(parse_generic_node),
      parse_symbol(Symbol::LeftBrace),
      many0(parse_implement_item_kind),
      parse_symbol(Symbol::RightBrace),
    )),
    |(visibility, _, trait_ident, generic, _, target, target_generic, _, item_list, _)| {
      ImplementTraitNode {
        visibility,
        target,
        target_generic,
        trait_ident,
        generic,
        item_list,
      }
    },
  )(s)
}

#[cfg(test)]
mod tests {
  use super::*;

  fn compile(s: &str) -> ImplementTraitNode {
    let (_, token_list) = lex(s).unwrap();
    match parse_implement_trait_node(Tokens::new(&token_list)) {
      Ok((_, node)) => node,
      Err(error) => {
        dbg!(error);
        panic!()
      }
    }
  }

  #[test]
  fn constant() {
    let source = "impl Foo for Bar {
      const BAZ: Bax = true;
    }";
    assert_eq!(
      compile(source),
      ImplementTraitNode {
        visibility: None,
        target: PathNode {
          ident_list: vec![IdentNode {
            raw: "Bar".to_owned()
          }]
        },
        target_generic: None,
        trait_ident: PathNode {
          ident_list: vec![IdentNode {
            raw: "Foo".to_owned()
          }]
        },
        generic: None,
        item_list: vec![ImplementItemKind::Constant(
          vec![],
          ConstantNode {
            visibility: None,
            ident: IdentNode {
              raw: "BAZ".to_owned()
            },
            ty: TypeKind::Path(
              ImmutablityKind::Yes,
              PathNode {
                ident_list: vec![IdentNode {
                  raw: "Bax".to_owned()
                }]
              }
            ),
            value: ExpressionKind::Path(PathNode {
              ident_list: vec![IdentNode {
                raw: "true".to_owned()
              }]
            }),
          }
        )]
      }
    );
  }

  #[test]
  fn function() {
    let source = "impl Foo for Bar {
      fn baz() { }
    }";
    assert_eq!(
      compile(source),
      ImplementTraitNode {
        visibility: None,
        target: PathNode {
          ident_list: vec![IdentNode {
            raw: "Bar".to_owned()
          }]
        },
        target_generic: None,
        trait_ident: PathNode {
          ident_list: vec![IdentNode {
            raw: "Foo".to_owned(),
          }]
        },
        generic: None,
        item_list: vec![ImplementItemKind::Function(
          vec![],
          FunctionNode {
            visibility: None,
            is_async: false,
            ident: IdentNode {
              raw: "baz".to_owned()
            },
            generic: None,
            self_type: None,
            argument_list: vec![],
            return_type: None,
            block: BlockNode {
              statement_list: vec![]
            },
          }
        )]
      }
    );
  }

  #[test]
  fn method() {
    let source = "impl Foo for Bar {
      fn baz(self) { }
    }";
    assert_eq!(
      compile(source),
      ImplementTraitNode {
        visibility: None,
        target: PathNode {
          ident_list: vec![IdentNode {
            raw: "Bar".to_owned()
          }]
        },
        target_generic: None,
        trait_ident: PathNode {
          ident_list: vec![IdentNode {
            raw: "Foo".to_owned(),
          }]
        },
        generic: None,
        item_list: vec![ImplementItemKind::Function(
          vec![],
          FunctionNode {
            visibility: None,
            is_async: false,
            ident: IdentNode {
              raw: "baz".to_owned()
            },
            generic: None,
            self_type: Some(ImmutablityKind::Yes),
            argument_list: vec![],
            return_type: None,
            block: BlockNode {
              statement_list: vec![]
            },
          }
        )]
      }
    );
  }
}
