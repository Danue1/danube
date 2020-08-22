use super::*;

pub(super) fn parse_implement_trait_node(s: Tokens) -> ParseResult<ImplementTraitNode> {
  map(
    tuple((
      opt(parse_visibility),
      parse_keyword(Keyword::Impl),
      parse_path_node,
      opt(parse_generic_node),
      parse_keyword(Keyword::For),
      parse_path_node,
      opt(parse_generic_node),
      parse_symbol(Symbol::LeftBrace),
      many1(parse_implement_item_node),
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
        item_list: vec![ImplementItemNode::Constant(ConstantNode {
          visibility: None,
          ident: IdentNode {
            raw: "BAZ".to_owned()
          },
          ty: TypeNode::Path(
            Immutablity::Yes,
            PathNode {
              ident_list: vec![IdentNode {
                raw: "Bax".to_owned()
              }]
            }
          ),
          value: ExpressionNode::Literal(LiteralValueNode::Bool(true)),
        })]
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
        item_list: vec![ImplementItemNode::Function(FunctionNode {
          visibility: None,
          is_async: false,
          ident: IdentNode {
            raw: "baz".to_owned()
          },
          generic: None,
          argument_list: vec![],
          return_type: None,
          body: vec![]
        })]
      }
    );
  }
}