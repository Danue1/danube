use super::*;

pub(super) fn parse_statement_kind(s: Tokens) -> ParseResult<StatementKind> {
  alt((
    map(parse_item_node, |node| StatementKind::Item(Box::new(node))),
    map(parse_compound_assign_node, |node| {
      StatementKind::CompoundAssign(Box::new(node))
    }),
    map(parse_let_node, |node| StatementKind::Let(Box::new(node))),
    map(parse_expression_kind, StatementKind::ExpressionKind),
    map(parse_symbol(Symbol::Semicolon), |_| {
      StatementKind::Semicolon
    }),
  ))(s)
}

#[cfg(test)]
mod tests {
  use super::*;

  fn compile(s: &str) -> StatementKind {
    let (_, token_list) = lex(s).unwrap();
    match parse_statement_kind(Tokens::new(&token_list)) {
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
      StatementKind::Item(Box::new(ItemNode {
        attribute_list: vec![],
        kind: ItemKind::Constant(ConstantNode {
          visibility: None,
          ident: IdentNode {
            raw: "FOO".to_owned(),
          },
          ty: TypeKind::Path(
            ImmutablityKind::Yes,
            PathNode {
              ident_list: vec![IdentNode {
                raw: "Foo".to_owned()
              }]
            }
          ),
          value: ExpressionKind::Literal(LiteralKind::Bool(true))
        })
      }))
    );
  }
}
