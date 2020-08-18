use super::*;

pub(super) fn parse_struct_node(s: Tokens) -> ParseResult<StructNode> {
  map(
    tuple((
      parse_keyword(Keyword::Struct),
      parse_ident_node,
      parse_struct_fields_node,
    )),
    |(_, ident, fields)| StructNode { ident, fields },
  )(s)
}

#[cfg(test)]
mod tests {
  use super::*;

  fn compile(s: &str) -> StructNode {
    let (_, token_list) = lex(s).unwrap();
    match parse_struct_node(Tokens::new(&token_list)) {
      Ok((_, node)) => node,
      Err(error) => {
        dbg!(error);
        panic!()
      }
    }
  }

  #[test]
  fn unnamed_single() {
    let source = "struct Foo(Bar);";
    assert_eq!(
      compile(source),
      StructNode {
        ident: IdentNode {
          raw: "Foo".to_owned()
        },
        fields: StructFieldsNode::Unnamed(StructUnnamedFieldsNode {
          node_list: vec![TypeNode::Ident(IdentNode {
            raw: "Bar".to_owned()
          })]
        })
      }
    );
  }

  #[test]
  fn unnamed_double() {
    let source = "struct Foo(Bar, Baz);";
    assert_eq!(
      compile(source),
      StructNode {
        ident: IdentNode {
          raw: "Foo".to_owned()
        },
        fields: StructFieldsNode::Unnamed(StructUnnamedFieldsNode {
          node_list: vec![
            TypeNode::Ident(IdentNode {
              raw: "Bar".to_owned()
            }),
            TypeNode::Ident(IdentNode {
              raw: "Baz".to_owned()
            })
          ]
        })
      }
    );
  }

  #[test]
  fn named_single() {
    let source = "struct Foo { bar: Bar };";
    assert_eq!(
      compile(source),
      StructNode {
        ident: IdentNode {
          raw: "Foo".to_owned()
        },
        fields: StructFieldsNode::Named(StructNamedFieldsNode {
          node_list: vec![(
            IdentNode {
              raw: "bar".to_owned()
            },
            TypeNode::Ident(IdentNode {
              raw: "Bar".to_owned()
            })
          )]
        })
      }
    );
  }

  #[test]
  fn named_double() {
    let source = "struct Foo { bar: Bar, baz: Baz };";
    assert_eq!(
      compile(source),
      StructNode {
        ident: IdentNode {
          raw: "Foo".to_owned()
        },
        fields: StructFieldsNode::Named(StructNamedFieldsNode {
          node_list: vec![
            (
              IdentNode {
                raw: "bar".to_owned()
              },
              TypeNode::Ident(IdentNode {
                raw: "Bar".to_owned()
              })
            ),
            (
              IdentNode {
                raw: "baz".to_owned()
              },
              TypeNode::Ident(IdentNode {
                raw: "Baz".to_owned()
              })
            ),
          ]
        })
      }
    );
  }
}
