use super::*;

pub(super) fn parse_let_node(s: Tokens) -> ParseResult<LetNode> {
  map(
    tuple((
      parse_keyword(Keyword::Let),
      parse_immutablity_kind,
      parse_pattern_kind,
      opt(map(
        tuple((parse_symbol(Symbol::Colon), parse_type_kind)),
        |(_, ty)| ty,
      )),
      opt(map(
        tuple((parse_symbol(Symbol::Assign), parse_expression_kind)),
        |(_, expression)| expression,
      )),
      parse_symbol(Symbol::Semicolon),
    )),
    |(_, immutablity, pattern, ty, value, _)| LetNode {
      immutablity,
      pattern,
      ty,
      value,
    },
  )(s)
}

#[cfg(test)]
mod immutable_tests {
  use super::*;

  fn compile(s: &str) -> LetNode {
    let (_, token_list) = lex(s).unwrap();
    match parse_let_node(Tokens::new(&token_list)) {
      Ok((_, node)) => node,
      Err(error) => {
        dbg!(error);
        panic!()
      }
    }
  }

  #[test]
  fn let_unassigned() {
    let source = "let foo;";
    assert_eq!(
      compile(source),
      LetNode {
        immutablity: ImmutablityKind::Yes,
        pattern: PatternKind::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "foo".to_owned()
          }]
        }),
        ty: None,
        value: None,
      }
    );
  }

  #[test]
  fn let_mut_unassigned() {
    let source = "let mut foo;";
    assert_eq!(
      compile(source),
      LetNode {
        immutablity: ImmutablityKind::Nope,
        pattern: PatternKind::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "foo".to_owned()
          }]
        }),
        ty: None,
        value: None,
      }
    );
  }

  #[test]
  fn untyped() {
    let source = "let foo = true;";
    assert_eq!(
      compile(source),
      LetNode {
        immutablity: ImmutablityKind::Yes,
        pattern: PatternKind::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "foo".to_owned()
          }]
        }),
        ty: None,
        value: Some(ExpressionKind::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "true".to_owned()
          }]
        })),
      }
    );
  }

  #[test]
  fn typed() {
    let source = "let foo: bool = true;";
    assert_eq!(
      compile(source),
      LetNode {
        immutablity: ImmutablityKind::Yes,
        pattern: PatternKind::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "foo".to_owned()
          }]
        }),
        ty: Some(TypeKind::Path(
          ImmutablityKind::Yes,
          PathNode {
            ident_list: vec![IdentNode {
              raw: "bool".to_owned()
            }]
          }
        )),
        value: Some(ExpressionKind::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "true".to_owned()
          }]
        })),
      }
    );
  }

  #[test]
  fn unnamed_implicit() {
    let source = "let (foo) = true;";
    assert_eq!(
      compile(source),
      LetNode {
        immutablity: ImmutablityKind::Yes,
        pattern: PatternKind::UnnamedStruct(UnnamedStructNode {
          path: None,
          field_list: vec![PatternKind::Path(PathNode {
            ident_list: vec![IdentNode {
              raw: "foo".to_owned()
            }]
          })]
        }),
        ty: None,
        value: Some(ExpressionKind::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "true".to_owned()
          }]
        })),
      }
    );
  }

  #[test]
  fn unnamed_single() {
    let source = "let Foo(foo) = true;";
    assert_eq!(
      compile(source),
      LetNode {
        immutablity: ImmutablityKind::Yes,
        pattern: PatternKind::UnnamedStruct(UnnamedStructNode {
          path: Some(PathNode {
            ident_list: vec![IdentNode {
              raw: "Foo".to_owned()
            }]
          }),
          field_list: vec![PatternKind::Path(PathNode {
            ident_list: vec![IdentNode {
              raw: "foo".to_owned()
            }]
          })]
        }),
        ty: None,
        value: Some(ExpressionKind::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "true".to_owned()
          }]
        })),
      }
    );
  }

  #[test]
  fn unnamed_double() {
    let source = "let Foo(foo, bar) = true;";
    assert_eq!(
      compile(source),
      LetNode {
        immutablity: ImmutablityKind::Yes,
        pattern: PatternKind::UnnamedStruct(UnnamedStructNode {
          path: Some(PathNode {
            ident_list: vec![IdentNode {
              raw: "Foo".to_owned()
            }]
          }),
          field_list: vec![
            PatternKind::Path(PathNode {
              ident_list: vec![IdentNode {
                raw: "foo".to_owned()
              }]
            }),
            PatternKind::Path(PathNode {
              ident_list: vec![IdentNode {
                raw: "bar".to_owned()
              }]
            })
          ]
        }),
        ty: None,
        value: Some(ExpressionKind::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "true".to_owned()
          }]
        })),
      }
    );
  }

  #[test]
  fn named_single_implicit() {
    let source = "let { foo } = true;";
    assert_eq!(
      compile(source),
      LetNode {
        immutablity: ImmutablityKind::Yes,
        pattern: PatternKind::NamedStruct(NamedStructNode {
          path: None,
          field_list: vec![FieldNode {
            ident: IdentNode {
              raw: "foo".to_owned()
            },
            pattern: None
          }]
        }),
        ty: None,
        value: Some(ExpressionKind::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "true".to_owned()
          }]
        })),
      }
    );
  }

  #[test]
  fn named_single() {
    let source = "let Foo { foo } = true;";
    assert_eq!(
      compile(source),
      LetNode {
        immutablity: ImmutablityKind::Yes,
        pattern: PatternKind::NamedStruct(NamedStructNode {
          path: Some(PathNode {
            ident_list: vec![IdentNode {
              raw: "Foo".to_owned()
            }]
          }),
          field_list: vec![FieldNode {
            ident: IdentNode {
              raw: "foo".to_owned()
            },
            pattern: None
          }]
        }),
        ty: None,
        value: Some(ExpressionKind::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "true".to_owned()
          }]
        })),
      }
    );
  }

  #[test]
  fn named_double() {
    let source = "let Foo { foo, bar } = true;";
    assert_eq!(
      compile(source),
      LetNode {
        immutablity: ImmutablityKind::Yes,
        pattern: PatternKind::NamedStruct(NamedStructNode {
          path: Some(PathNode {
            ident_list: vec![IdentNode {
              raw: "Foo".to_owned()
            }]
          }),
          field_list: vec![
            FieldNode {
              ident: IdentNode {
                raw: "foo".to_owned()
              },
              pattern: None
            },
            FieldNode {
              ident: IdentNode {
                raw: "bar".to_owned()
              },
              pattern: None
            }
          ]
        }),
        ty: None,
        value: Some(ExpressionKind::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "true".to_owned()
          }]
        })),
      }
    );
  }
}

#[cfg(test)]
mod mutable_tests {
  use super::*;

  fn compile(s: &str) -> LetNode {
    let (_, token_list) = lex(s).unwrap();
    match parse_let_node(Tokens::new(&token_list)) {
      Ok((_, node)) => node,
      Err(error) => {
        dbg!(error);
        panic!()
      }
    }
  }

  #[test]
  fn untyped() {
    let source = "let mut foo = true;";
    assert_eq!(
      compile(source),
      LetNode {
        immutablity: ImmutablityKind::Nope,
        pattern: PatternKind::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "foo".to_owned()
          }]
        }),
        ty: None,
        value: Some(ExpressionKind::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "true".to_owned()
          }]
        })),
      }
    );
  }

  #[test]
  fn typed() {
    let source = "let mut foo: bool = true;";
    assert_eq!(
      compile(source),
      LetNode {
        immutablity: ImmutablityKind::Nope,
        pattern: PatternKind::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "foo".to_owned()
          }]
        }),
        ty: Some(TypeKind::Path(
          ImmutablityKind::Yes,
          PathNode {
            ident_list: vec![IdentNode {
              raw: "bool".to_owned()
            }]
          }
        )),
        value: Some(ExpressionKind::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "true".to_owned()
          }]
        })),
      }
    );
  }

  #[test]
  fn pattern_unnamed() {
    let source = "let mut Foo::Bar = true;";
    assert_eq!(
      compile(source),
      LetNode {
        immutablity: ImmutablityKind::Nope,
        pattern: PatternKind::Path(PathNode {
          ident_list: vec![
            IdentNode {
              raw: "Foo".to_owned()
            },
            IdentNode {
              raw: "Bar".to_owned()
            }
          ]
        }),
        ty: None,
        value: Some(ExpressionKind::Path(PathNode {
          ident_list: vec![IdentNode {
            raw: "true".to_owned()
          }]
        })),
      }
    );
  }
}
