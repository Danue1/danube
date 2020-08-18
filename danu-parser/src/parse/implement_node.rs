use super::*;

pub(super) fn parse_implement_node(s: Tokens) -> ParseResult<ImplementNode> {
  map(
    tuple((
      parse_keyword(Keyword::Impl),
      parse_ident_node,
      opt(parse_generic_node),
      parse_symbol(Symbol::LeftBrace),
      many1(parse_implement_item_node),
      parse_symbol(Symbol::RightBrace),
    )),
    |(_, target, generic, _, item_list, _)| ImplementNode {
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
        target: IdentNode {
          raw: "Foo".to_owned()
        },
        generic: None,
        item_list: vec![ImplementItemNode::Constant(ConstantNode {
          ident: IdentNode {
            raw: "BAR".to_owned()
          },
          ty: TypeNode::Ident(IdentNode {
            raw: "Baz".to_owned()
          }),
          value: ExpressionNode::Literal(LiteralValueNode::Bool(true)),
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
        target: IdentNode {
          raw: "Foo".to_owned()
        },
        generic: None,
        item_list: vec![ImplementItemNode::Function(FunctionNode {
          ident: IdentNode {
            raw: "bar".to_owned()
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
