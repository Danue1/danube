use super::*;

pub(super) fn parse_statement_node(s: Tokens) -> ParseResult<StatementNode> {
  alt((
    map(parse_item_node, |node| StatementNode::Item(Box::new(node))),
    map(parse_compound_assign_node, |node| {
      StatementNode::CompoundAssign(Box::new(node))
    }),
    map(parse_let_node, |node| StatementNode::Let(Box::new(node))),
    map(parse_expression_node, StatementNode::Expression),
    map(parse_symbol(Symbol::Semicolon), |_| {
      StatementNode::Semicolon
    }),
  ))(s)
}

#[cfg(test)]
mod tests {
  use super::*;

  fn compile(s: &str) -> StatementNode {
    let (_, token_list) = lex(s).unwrap();
    match parse_statement_node(Tokens::new(&token_list)) {
      Ok((_, node)) => node,
      Err(error) => {
        dbg!(error);
        panic!()
      }
    }
  }

  #[test]
  fn statement_item_constant() {
    let source = "const FOO: Foo = true;";
    assert_eq!(
      compile(source),
      StatementNode::Item(Box::new(ItemNode::Constant(ConstantNode {
        visibility: None,
        ident: IdentNode {
          raw: "FOO".to_owned(),
        },
        ty: TypeNode::Path(
          Immutablity::Yes,
          PathNode {
            ident_list: vec![IdentNode {
              raw: "Foo".to_owned()
            }]
          }
        ),
        value: ExpressionNode::Literal(LiteralValueNode::Bool(true))
      })))
    );
  }
}
