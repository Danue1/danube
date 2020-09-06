use super::*;

pub(super) fn parse_implement_node(s: Tokens) -> ParseResult<ImplementNode> {
  map(
    tuple((
      opt(parse_visibility_kind),
      parse_keyword(Keyword::Impl),
      parse_path_node,
      opt(parse_generic_node),
      parse_symbol(Symbol::LeftBrace),
      many1(parse_implement_item_kind),
      parse_symbol(Symbol::RightBrace),
    )),
    |(visibility, _, target, generic, _, item_list, _)| ImplementNode {
      visibility,
      target,
      generic,
      item_list,
    },
  )(s)
}

#[cfg(test)]
mod tests {
  use super::*;

  fn compile(s: &str) -> ImplementNode {
    let (_, token_list) = lex(s).unwrap();
    match parse_implement_node(Tokens::new(&token_list)) {
      Ok((_, node)) => node,
      Err(error) => {
        dbg!(error);
        panic!()
      }
    }
  }

  #[test]
  fn constant() {
    let source = "impl Foo {
      const BAR: Baz = true;
    }";
    assert_eq!(
      compile(source),
      ImplementNode {
        visibility: None,
        target: PathNode {
          ident_list: vec![IdentNode {
            raw: "Foo".to_owned()
          }]
        },
        generic: None,
        item_list: vec![ImplementItemKind::Constant(ConstantNode {
          visibility: None,
          ident: IdentNode {
            raw: "BAR".to_owned()
          },
          ty: TypeKind::Path(
            ImmutablityKind::Yes,
            PathNode {
              ident_list: vec![IdentNode {
                raw: "Baz".to_owned()
              }]
            }
          ),
          value: ExpressionKind::Literal(LiteralValueKind::Bool(true)),
        })]
      }
    );
  }

  #[test]
  fn function() {
    let source = "impl Foo {
      fn bar() { }
    }";
    assert_eq!(
      compile(source),
      ImplementNode {
        visibility: None,
        target: PathNode {
          ident_list: vec![IdentNode {
            raw: "Foo".to_owned()
          }]
        },
        generic: None,
        item_list: vec![ImplementItemKind::Function(FunctionNode {
          visibility: None,
          is_async: false,
          ident: IdentNode {
            raw: "bar".to_owned()
          },
          generic: None,
          self_type: None,
          argument_list: vec![],
          return_type: None,
          block: BlockNode {
            statement_list: vec![]
          },
        })]
      }
    );
  }
}
