use super::*;

pub(super) fn parse_loop_node(s: Tokens) -> ParseResult<LoopNode> {
  map(
    tuple((
      parse_keyword(Keyword::Loop),
      parse_symbol(Symbol::LeftBrace),
      many0(parse_statement_node),
      parse_symbol(Symbol::RightBrace),
    )),
    |(_, _, body, _)| LoopNode { body },
  )(s)
}

#[cfg(test)]
mod tests {
  use super::*;

  fn compile(s: &str) -> LoopNode {
    let (_, token_list) = lex(s).unwrap();
    match parse_loop_node(Tokens::new(&token_list)) {
      Ok((_, node)) => node,
      Err(error) => {
        dbg!(error);
        panic!()
      }
    }
  }

  #[test]
  fn test() {
    let source = "loop { }";
    assert_eq!(compile(source), LoopNode { body: vec![] });
  }
}
