use super::*;

pub(super) fn parse_module_node(s: Tokens) -> ParseResult<ModuleNode> {
  map(
    tuple((
      parse_keyword(Keyword::Module),
      parse_ident_node,
      alt((
        map(parse_symbol(Symbol::Semicolon), |_| vec![]),
        map(
          tuple((
            parse_symbol(Symbol::LeftBrace),
            many0(parse_item_node),
            parse_symbol(Symbol::RightBrace),
          )),
          |(_, item_list, _)| item_list,
        ),
      )),
    )),
    |(_, ident, item_list)| ModuleNode { ident, item_list },
  )(s)
}

#[cfg(test)]
mod tests {
  use super::*;

  fn compile(s: &str) -> ModuleNode {
    let (_, token_list) = lex(s).unwrap();
    match parse_module_node(Tokens::new(&token_list)) {
      Ok((_, node)) => node,
      Err(error) => {
        dbg!(error);
        panic!()
      }
    }
  }

  #[test]
  fn shortcut() {
    let source = "mod foo;";
    assert_eq!(
      compile(source),
      ModuleNode {
        ident: IdentNode {
          raw: "foo".to_owned()
        },
        item_list: vec![],
      }
    );
  }

  #[test]
  fn longcut() {
    let source = "mod foo { }";
    assert_eq!(
      compile(source),
      ModuleNode {
        ident: IdentNode {
          raw: "foo".to_owned()
        },
        item_list: vec![],
      }
    );
  }
}
