use super::*;

pub(super) fn parse_implement_trait_node(s: Tokens) -> ParseResult<ImplementTraitNode> {
  map(
    tuple((
      parse_keyword(Keyword::Impl),
      parse_ident_node,
      parse_keyword(Keyword::For),
      parse_ident_node,
      parse_symbol(Symbol::LeftBrace),
      many1(parse_implement_item_node),
      parse_symbol(Symbol::RightBrace),
    )),
    |(_, trait_ident, _, target, _, item_list, _)| ImplementTraitNode {
      target,
      trait_ident,
      item_list,
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
        target: IdentNode {
          raw: "Bar".to_owned()
        },
        trait_ident: IdentNode {
          raw: "Foo".to_owned()
        },
        item_list: vec![ImplementItemNode::Constant(ConstantNode {
          ident: IdentNode {
            raw: "BAZ".to_owned()
          },
          ty: TypeNode::Ident(IdentNode {
            raw: "Bax".to_owned()
          }),
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
        target: IdentNode {
          raw: "Bar".to_owned()
        },
        trait_ident: IdentNode {
          raw: "Foo".to_owned(),
        },
        item_list: vec![ImplementItemNode::Function(FunctionNode {
          ident: IdentNode {
            raw: "baz".to_owned()
          },
          argument_list: vec![],
          return_type: None,
          body: vec![]
        })]
      }
    );
  }
}
