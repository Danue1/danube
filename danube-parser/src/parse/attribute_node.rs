use super::*;

pub(super) fn parse_attribute_node(s: Tokens) -> ParseResult<AttributeNode> {
  map(
    tuple((
      parse_symbol(Symbol::Hashtag),
      parse_symbol(Symbol::LeftBracket),
      parse_path_node,
      parse_symbol(Symbol::RightBracket),
    )),
    |(_, _, path, _)| AttributeNode { path },
  )(s)
}

#[cfg(test)]
mod tests {
  use super::*;

  fn compile(s: &str) -> AttributeNode {
    let (_, token_list) = lex(s).unwrap();
    match parse_attribute_node(Tokens::new(&token_list)) {
      Ok((_, node)) => node,
      Err(error) => {
        dbg!(error);
        panic!()
      }
    }
  }

  #[test]
  fn attribute_native() {
    let source = "#[native]";
    assert_eq!(
      compile(source),
      AttributeNode {
        path: PathNode {
          ident_list: vec![IdentNode {
            raw: "native".to_owned()
          }]
        },
      }
    );
  }
}
