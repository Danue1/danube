use super::*;

pub(super) fn parse_feature_node(s: Tokens) -> ParseResult<FeatureNode> {
  map(
    tuple((
      parse_symbol(Symbol::Hashtag),
      parse_symbol(Symbol::Not),
      parse_symbol(Symbol::LeftBracket),
      parse_ident_node,
      parse_symbol(Symbol::RightBracket),
    )),
    |(_, _, _, name, _)| FeatureNode { name },
  )(s)
}

#[cfg(test)]
mod tests {
  use super::*;

  fn compile(s: &str) -> FeatureNode {
    let (_, token_list) = lex(s).unwrap();
    match parse_feature_node(Tokens::new(&token_list)) {
      Ok((_, node)) => node,
      Err(error) => {
        dbg!(error);
        panic!()
      }
    }
  }

  #[test]
  fn single() {
    let source = "#![feature]";
    assert_eq!(
      compile(source),
      FeatureNode {
        name: IdentNode {
          raw: "feature".to_owned(),
        },
      }
    );
  }
}
