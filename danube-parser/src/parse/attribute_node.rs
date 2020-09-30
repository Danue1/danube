use super::*;
use std::collections::HashMap;
use std::iter::FromIterator;

pub(super) fn parse_attribute_node(s: Tokens) -> ParseResult<AttributeNode> {
  map(
    tuple((
      parse_symbol(Symbol::Hashtag),
      parse_symbol(Symbol::LeftBracket),
      parse_path_node,
      opt(parse_attribute_argument_list),
      parse_symbol(Symbol::RightBracket),
    )),
    |(_, _, path, args, _)| AttributeNode {
      path,
      args: args.unwrap_or_default(),
    },
  )(s)
}

fn parse_attribute_argument_list(s: Tokens) -> ParseResult<HashMap<String, Option<LiteralKind>>> {
  map(
    tuple((
      parse_symbol(Symbol::LeftParens),
      separated_list(parse_symbol(Symbol::Comma), parse_attribute_argument_node),
      opt(parse_symbol(Symbol::Comma)),
      parse_symbol(Symbol::RightParens),
    )),
    |(_, args, _, _)| HashMap::from_iter(args),
  )(s)
}

fn parse_attribute_argument_node(s: Tokens) -> ParseResult<(String, Option<LiteralKind>)> {
  tuple((
    parse_identifier,
    opt(preceded(parse_symbol(Symbol::Assign), parse_literal_kind)),
  ))(s)
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
        args: HashMap::new()
      }
    );
  }

  #[test]
  fn attribute_primitive() {
    let source = r#"#[primitive(name = "Int")]"#;
    assert_eq!(
      compile(source),
      AttributeNode {
        path: PathNode {
          ident_list: vec![IdentNode {
            raw: "primitive".to_owned()
          }]
        },
        args: HashMap::from_iter(vec![(
          "name".to_owned(),
          Some(LiteralKind::String("Int".to_owned()))
        )])
      }
    );
  }
}
