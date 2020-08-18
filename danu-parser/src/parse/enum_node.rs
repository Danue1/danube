use super::*;

pub(super) fn parse_enum_node(s: Tokens) -> ParseResult<EnumNode> {
  map(
    tuple((
      parse_keyword(Keyword::Enum),
      parse_ident_node,
      parse_symbol(Symbol::Assign),
      opt(parse_symbol(Symbol::BitOr)),
      separated_nonempty_list(parse_symbol(Symbol::BitOr), parse_enum_variant_node),
      parse_symbol(Symbol::Semicolon),
    )),
    |(_, ident, _, _, variant_list, _)| EnumNode {
      ident,
      variant_list,
    },
  )(s)
}

#[cfg(test)]
mod tests {
  use super::*;

  fn compile(s: &str) -> EnumNode {
    let (_, token_list) = lex(s).unwrap();
    match parse_enum_node(Tokens::new(&token_list)) {
      Ok((_, node)) => node,
      Err(error) => {
        dbg!(error);
        panic!()
      }
    }
  }

  #[test]
  fn single() {
    let source = "enum Foo = Bar;";
    assert_eq!(
      compile(source),
      EnumNode {
        ident: IdentNode {
          raw: "Foo".to_owned()
        },
        variant_list: vec![EnumVariantNode {
          ident: IdentNode {
            raw: "Bar".to_owned()
          },
          ty: None
        }]
      }
    );
  }

  #[test]
  fn double() {
    let source = "enum Foo = Bar | Baz;";
    assert_eq!(
      compile(source),
      EnumNode {
        ident: IdentNode {
          raw: "Foo".to_owned()
        },
        variant_list: vec![
          EnumVariantNode {
            ident: IdentNode {
              raw: "Bar".to_owned()
            },
            ty: None,
          },
          EnumVariantNode {
            ident: IdentNode {
              raw: "Baz".to_owned()
            },
            ty: None
          }
        ]
      }
    );
  }

  #[test]
  fn pipe_single() {
    let source = "enum Foo = | Bar;";
    assert_eq!(
      compile(source),
      EnumNode {
        ident: IdentNode {
          raw: "Foo".to_owned()
        },
        variant_list: vec![EnumVariantNode {
          ident: IdentNode {
            raw: "Bar".to_owned()
          },
          ty: None
        }]
      }
    );
  }

  #[test]
  fn pipe_double() {
    let source = "enum Foo = | Bar | Baz;";
    assert_eq!(
      compile(source),
      EnumNode {
        ident: IdentNode {
          raw: "Foo".to_owned()
        },
        variant_list: vec![
          EnumVariantNode {
            ident: IdentNode {
              raw: "Bar".to_owned()
            },
            ty: None,
          },
          EnumVariantNode {
            ident: IdentNode {
              raw: "Baz".to_owned()
            },
            ty: None
          }
        ]
      }
    );
  }
}
