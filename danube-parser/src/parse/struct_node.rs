use super::*;

pub(super) fn parse_struct_node(s: Tokens) -> ParseResult<StructNode> {
  map(
    tuple((
      opt(parse_visibility_kind),
      parse_keyword(Keyword::Struct),
      parse_ident_node,
      opt(parse_generic_node),
      parse_struct_fields_kind,
    )),
    |(visibility, _, ident, generic, fields)| StructNode {
      visibility,
      ident,
      generic,
      fields,
    },
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
        visibility: None,
        ident: IdentNode {
          raw: "Foo".to_owned()
        },
        generic: None,
        fields: StructFieldsKind::Unnamed(StructUnnamedFieldsNode {
          node_list: vec![TypeKind::Path(
            ImmutablityKind::Yes,
            PathNode {
              ident_list: vec![IdentNode {
                raw: "Bar".to_owned()
              }]
            }
          )]
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
        visibility: None,
        ident: IdentNode {
          raw: "Foo".to_owned()
        },
        generic: None,
        fields: StructFieldsKind::Unnamed(StructUnnamedFieldsNode {
          node_list: vec![
            TypeKind::Path(
              ImmutablityKind::Yes,
              PathNode {
                ident_list: vec![IdentNode {
                  raw: "Bar".to_owned()
                }]
              }
            ),
            TypeKind::Path(
              ImmutablityKind::Yes,
              PathNode {
                ident_list: vec![IdentNode {
                  raw: "Baz".to_owned()
                }]
              }
            )
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
        visibility: None,
        ident: IdentNode {
          raw: "Foo".to_owned()
        },
        generic: None,
        fields: StructFieldsKind::Named(StructNamedFieldsNode {
          node_list: vec![(
            IdentNode {
              raw: "bar".to_owned()
            },
            TypeKind::Path(
              ImmutablityKind::Yes,
              PathNode {
                ident_list: vec![IdentNode {
                  raw: "Bar".to_owned()
                }]
              }
            )
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
        visibility: None,
        ident: IdentNode {
          raw: "Foo".to_owned()
        },
        generic: None,
        fields: StructFieldsKind::Named(StructNamedFieldsNode {
          node_list: vec![
            (
              IdentNode {
                raw: "bar".to_owned()
              },
              TypeKind::Path(
                ImmutablityKind::Yes,
                PathNode {
                  ident_list: vec![IdentNode {
                    raw: "Bar".to_owned()
                  }]
                }
              )
            ),
            (
              IdentNode {
                raw: "baz".to_owned()
              },
              TypeKind::Path(
                ImmutablityKind::Yes,
                PathNode {
                  ident_list: vec![IdentNode {
                    raw: "Baz".to_owned()
                  }]
                }
              )
            ),
          ]
        })
      }
    );
  }

  #[test]
  fn unnamed_generic() {
    let source = "struct Foo<T>(T);";
    assert_eq!(
      compile(source),
      StructNode {
        visibility: None,
        ident: IdentNode {
          raw: "Foo".to_owned()
        },
        generic: Some(GenericNode {
          path: PathNode {
            ident_list: vec![IdentNode {
              raw: "T".to_owned()
            }]
          },
          trait_list: vec![],
          default_trait_list: vec![],
        }),
        fields: StructFieldsKind::Unnamed(StructUnnamedFieldsNode {
          node_list: vec![TypeKind::Path(
            ImmutablityKind::Yes,
            PathNode {
              ident_list: vec![IdentNode {
                raw: "T".to_owned()
              }]
            }
          )]
        })
      }
    );
  }

  #[test]
  fn unnamed_generic_a_trait() {
    let source = "struct Foo<T: Foo>(T);";
    assert_eq!(
      compile(source),
      StructNode {
        visibility: None,
        ident: IdentNode {
          raw: "Foo".to_owned()
        },
        generic: Some(GenericNode {
          path: PathNode {
            ident_list: vec![IdentNode {
              raw: "T".to_owned()
            }]
          },
          trait_list: vec![PathNode {
            ident_list: vec![IdentNode {
              raw: "Foo".to_owned()
            }]
          }],
          default_trait_list: vec![],
        }),
        fields: StructFieldsKind::Unnamed(StructUnnamedFieldsNode {
          node_list: vec![TypeKind::Path(
            ImmutablityKind::Yes,
            PathNode {
              ident_list: vec![IdentNode {
                raw: "T".to_owned()
              }]
            }
          )]
        })
      }
    );
  }

  #[test]
  fn unnamed_generic_two_trait() {
    let source = "struct Foo<T: Foo + Bar>(T);";
    assert_eq!(
      compile(source),
      StructNode {
        visibility: None,
        ident: IdentNode {
          raw: "Foo".to_owned()
        },
        generic: Some(GenericNode {
          path: PathNode {
            ident_list: vec![IdentNode {
              raw: "T".to_owned()
            }]
          },
          trait_list: vec![
            PathNode {
              ident_list: vec![IdentNode {
                raw: "Foo".to_owned()
              }]
            },
            PathNode {
              ident_list: vec![IdentNode {
                raw: "Bar".to_owned()
              }]
            }
          ],
          default_trait_list: vec![],
        }),
        fields: StructFieldsKind::Unnamed(StructUnnamedFieldsNode {
          node_list: vec![TypeKind::Path(
            ImmutablityKind::Yes,
            PathNode {
              ident_list: vec![IdentNode {
                raw: "T".to_owned()
              }]
            }
          )]
        })
      }
    );
  }
}
