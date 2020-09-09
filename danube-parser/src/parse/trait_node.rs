use super::*;

pub(super) fn parse_trait_node(s: Tokens) -> ParseResult<TraitNode> {
  map(
    tuple((
      opt(parse_visibility_kind),
      parse_keyword(Keyword::Trait),
      parse_ident_node,
      opt(parse_generic_node),
      parse_inheritance,
      parse_symbol(Symbol::LeftBrace),
      many1(parse_trait_item_kind),
      parse_symbol(Symbol::RightBrace),
    )),
    |(visibility, _, ident, generic, inheritances, _, item_list, _)| TraitNode {
      visibility,
      ident,
      generic,
      inheritances,
      item_list,
    },
  )(s)
}

fn parse_inheritance(s: Tokens) -> ParseResult<Vec<(PathNode, Vec<PathNode>)>> {
  alt((
    preceded(
      parse_symbol(Symbol::Colon),
      separated_list(
        parse_symbol(Symbol::Comma),
        tuple((
          parse_path_node,
          alt((
            map(
              tuple((
                parse_symbol(Symbol::LessThan),
                separated_nonempty_list(parse_symbol(Symbol::Comma), parse_path_node),
                parse_symbol(Symbol::GreaterThan),
              )),
              |(_, path_list, _)| path_list,
            ),
            |s| Ok((s, vec![])),
          )),
        )),
      ),
    ),
    |s| Ok((s, vec![])),
  ))(s)
}

#[cfg(test)]
mod test_inheritance {
  use super::*;

  fn compile(s: &str) -> TraitNode {
    let (_, token_list) = lex(s).unwrap();
    match parse_trait_node(Tokens::new(&token_list)) {
      Ok((_, node)) => node,
      Err(error) => {
        dbg!(error);
        panic!()
      }
    }
  }

  #[test]
  fn a_inheritance() {
    let source = "trait Foo: Sized {
      type Foo;
    }";
    assert_eq!(
      compile(source),
      TraitNode {
        visibility: None,
        ident: IdentNode {
          raw: "Foo".to_owned()
        },
        generic: None,
        inheritances: vec![(
          PathNode {
            ident_list: vec![IdentNode {
              raw: "Sized".to_owned()
            }]
          },
          vec![]
        )],
        item_list: vec![TraitItemKind::OutputType(TraitItemOutputTypeNode {
          ident: IdentNode {
            raw: "Foo".to_owned()
          },
          ty: None
        })],
      }
    );
  }

  #[test]
  fn two_inheritance() {
    let source = "trait Foo: Sized, ops::Add<T> {
      type Foo;
    }";
    assert_eq!(
      compile(source),
      TraitNode {
        visibility: None,
        ident: IdentNode {
          raw: "Foo".to_owned()
        },
        generic: None,
        inheritances: vec![
          (
            PathNode {
              ident_list: vec![IdentNode {
                raw: "Sized".to_owned()
              }]
            },
            vec![]
          ),
          (
            PathNode {
              ident_list: vec![
                IdentNode {
                  raw: "ops".to_owned()
                },
                IdentNode {
                  raw: "Add".to_owned()
                }
              ]
            },
            vec![PathNode {
              ident_list: vec![IdentNode {
                raw: "T".to_owned()
              }]
            }]
          )
        ],
        item_list: vec![TraitItemKind::OutputType(TraitItemOutputTypeNode {
          ident: IdentNode {
            raw: "Foo".to_owned()
          },
          ty: None
        })],
      }
    );
  }
}

#[cfg(test)]
mod test_type {
  use super::*;

  fn compile(s: &str) -> TraitNode {
    let (_, token_list) = lex(s).unwrap();
    match parse_trait_node(Tokens::new(&token_list)) {
      Ok((_, node)) => node,
      Err(error) => {
        dbg!(error);
        panic!()
      }
    }
  }

  #[test]
  fn no_type() {
    let source = "trait Foo {
        type Foo;
      }";
    assert_eq!(
      compile(source),
      TraitNode {
        visibility: None,
        ident: IdentNode {
          raw: "Foo".to_owned()
        },
        generic: None,
        inheritances: vec![],
        item_list: vec![TraitItemKind::OutputType(TraitItemOutputTypeNode {
          ident: IdentNode {
            raw: "Foo".to_owned()
          },
          ty: None
        })],
      }
    );
  }

  #[test]
  fn typed() {
    let source = "trait Foo {
      type Foo = Bar;
    }";
    assert_eq!(
      compile(source),
      TraitNode {
        visibility: None,
        ident: IdentNode {
          raw: "Foo".to_owned()
        },
        generic: None,
        inheritances: vec![],
        item_list: vec![TraitItemKind::OutputType(TraitItemOutputTypeNode {
          ident: IdentNode {
            raw: "Foo".to_owned()
          },
          ty: Some(TypeKind::Path(
            ImmutablityKind::Yes,
            PathNode {
              ident_list: vec![IdentNode {
                raw: "Bar".to_owned()
              }]
            }
          ))
        })],
      }
    );
  }

  #[test]
  fn type_self() {
    let source = "trait Foo {
      type Bar = Self;
    }";
    assert_eq!(
      compile(source),
      TraitNode {
        visibility: None,
        ident: IdentNode {
          raw: "Foo".to_owned()
        },
        generic: None,
        inheritances: vec![],
        item_list: vec![TraitItemKind::OutputType(TraitItemOutputTypeNode {
          ident: IdentNode {
            raw: "Bar".to_owned()
          },
          ty: Some(TypeKind::Path(
            ImmutablityKind::Yes,
            PathNode {
              ident_list: vec![IdentNode {
                raw: "Self".to_owned()
              }]
            }
          ))
        })],
      }
    );
  }

  #[test]
  fn generic() {
    let source = "trait Foo {
      type Foo = Foo::Bar<Baz>;
    }";
    assert_eq!(
      compile(source),
      TraitNode {
        visibility: None,
        ident: IdentNode {
          raw: "Foo".to_owned()
        },
        generic: None,
        inheritances: vec![],
        item_list: vec![TraitItemKind::OutputType(TraitItemOutputTypeNode {
          ident: IdentNode {
            raw: "Foo".to_owned()
          },
          ty: Some(TypeKind::Generic(
            ImmutablityKind::Yes,
            PathNode {
              ident_list: vec![
                IdentNode {
                  raw: "Foo".to_owned()
                },
                IdentNode {
                  raw: "Bar".to_owned()
                }
              ]
            },
            vec![PathNode {
              ident_list: vec![IdentNode {
                raw: "Baz".to_owned()
              }]
            }]
          ))
        })],
      }
    );
  }
}

#[cfg(test)]
mod test_constant {
  use super::*;

  fn compile(s: &str) -> TraitNode {
    let (_, token_list) = lex(s).unwrap();
    match parse_trait_node(Tokens::new(&token_list)) {
      Ok((_, node)) => node,
      Err(error) => {
        dbg!(error);
        panic!()
      }
    }
  }

  #[test]
  fn no_default_value() {
    let source = "trait Foo {
      const FOO: Bar;
    }";
    assert_eq!(
      compile(source),
      TraitNode {
        visibility: None,
        ident: IdentNode {
          raw: "Foo".to_owned()
        },
        generic: None,
        inheritances: vec![],
        item_list: vec![TraitItemKind::Constant(TraitItemConstantNode {
          ident: IdentNode {
            raw: "FOO".to_owned()
          },
          ty: TypeKind::Path(
            ImmutablityKind::Yes,
            PathNode {
              ident_list: vec![IdentNode {
                raw: "Bar".to_owned()
              }]
            }
          ),
          default_value: None,
        })]
      }
    );
  }

  #[test]
  fn a_default_value() {
    let source = "trait Foo {
      const FOO: Bar = true;
    }";
    assert_eq!(
      compile(source),
      TraitNode {
        visibility: None,
        ident: IdentNode {
          raw: "Foo".to_owned()
        },
        generic: None,
        inheritances: vec![],
        item_list: vec![TraitItemKind::Constant(TraitItemConstantNode {
          ident: IdentNode {
            raw: "FOO".to_owned()
          },
          ty: TypeKind::Path(
            ImmutablityKind::Yes,
            PathNode {
              ident_list: vec![IdentNode {
                raw: "Bar".to_owned()
              }]
            }
          ),
          default_value: Some(LiteralKind::Bool(true)),
        })]
      }
    );
  }
}

#[cfg(test)]
mod test_function {
  use super::*;

  fn compile(s: &str) -> TraitNode {
    let (_, token_list) = lex(s).unwrap();
    match parse_trait_node(Tokens::new(&token_list)) {
      Ok((_, node)) => node,
      Err(error) => {
        dbg!(error);
        panic!()
      }
    }
  }

  #[test]
  fn no_argument_no_body() {
    let source = "trait Foo {
      fn foo();
    }";
    assert_eq!(
      compile(source),
      TraitNode {
        visibility: None,
        ident: IdentNode {
          raw: "Foo".to_owned()
        },
        generic: None,
        inheritances: vec![],
        item_list: vec![TraitItemKind::Function(TraitItemFunctionNode {
          is_async: false,
          ident: IdentNode {
            raw: "foo".to_owned()
          },
          generic: None,
          self_type: None,
          argument_list: vec![],
          return_type: None,
          block: None,
        })]
      }
    );
  }

  #[test]
  fn a_argument_no_body() {
    let source = "trait Foo {
      fn foo(bar: Bar);
    }";
    assert_eq!(
      compile(source),
      TraitNode {
        visibility: None,
        ident: IdentNode {
          raw: "Foo".to_owned()
        },
        generic: None,
        inheritances: vec![],
        item_list: vec![TraitItemKind::Function(TraitItemFunctionNode {
          is_async: false,
          ident: IdentNode {
            raw: "foo".to_owned()
          },
          generic: None,
          self_type: None,
          argument_list: vec![FunctionArgumentNode {
            immutablity: ImmutablityKind::Yes,
            ident: IdentNode {
              raw: "bar".to_owned()
            },
            ty: TypeKind::Path(
              ImmutablityKind::Yes,
              PathNode {
                ident_list: vec![IdentNode {
                  raw: "Bar".to_owned()
                }]
              }
            )
          }],
          return_type: None,
          block: None,
        })]
      }
    );
  }

  #[test]
  fn two_argument_no_body() {
    let source = "trait Foo {
      fn foo(bar: Bar, baz: Baz);
    }";
    assert_eq!(
      compile(source),
      TraitNode {
        visibility: None,
        ident: IdentNode {
          raw: "Foo".to_owned()
        },
        generic: None,
        inheritances: vec![],
        item_list: vec![TraitItemKind::Function(TraitItemFunctionNode {
          is_async: false,
          ident: IdentNode {
            raw: "foo".to_owned()
          },
          generic: None,
          self_type: None,
          argument_list: vec![
            FunctionArgumentNode {
              immutablity: ImmutablityKind::Yes,
              ident: IdentNode {
                raw: "bar".to_owned()
              },
              ty: TypeKind::Path(
                ImmutablityKind::Yes,
                PathNode {
                  ident_list: vec![IdentNode {
                    raw: "Bar".to_owned()
                  }]
                }
              )
            },
            FunctionArgumentNode {
              immutablity: ImmutablityKind::Yes,
              ident: IdentNode {
                raw: "baz".to_owned()
              },
              ty: TypeKind::Path(
                ImmutablityKind::Yes,
                PathNode {
                  ident_list: vec![IdentNode {
                    raw: "Baz".to_owned()
                  }]
                }
              )
            }
          ],
          return_type: None,
          block: None
        })]
      }
    );
  }

  #[test]
  fn no_argument_a_body() {
    let source = "trait Foo {
      fn foo() {
        const FOO: bool = true;
      }
    }";
    assert_eq!(
      compile(source),
      TraitNode {
        visibility: None,
        ident: IdentNode {
          raw: "Foo".to_owned()
        },
        generic: None,
        inheritances: vec![],
        item_list: vec![TraitItemKind::Function(TraitItemFunctionNode {
          is_async: false,
          ident: IdentNode {
            raw: "foo".to_owned()
          },
          generic: None,
          self_type: None,
          argument_list: vec![],
          return_type: None,
          block: Some(BlockNode {
            statement_list: vec![StatementKind::Item(Box::new(ItemNode {
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
                      raw: "bool".to_owned(),
                    }]
                  }
                ),
                value: ExpressionKind::Literal(LiteralKind::Bool(true)),
              })
            }))]
          }),
        })]
      }
    );
  }

  #[test]
  fn a_argument_a_body() {
    let source = "trait Foo {
      fn foo(bar: Bar) {
        const FOO: bool = true;
      }
    }";
    assert_eq!(
      compile(source),
      TraitNode {
        visibility: None,
        ident: IdentNode {
          raw: "Foo".to_owned()
        },
        generic: None,
        inheritances: vec![],
        item_list: vec![TraitItemKind::Function(TraitItemFunctionNode {
          is_async: false,
          ident: IdentNode {
            raw: "foo".to_owned()
          },
          generic: None,
          self_type: None,
          argument_list: vec![FunctionArgumentNode {
            immutablity: ImmutablityKind::Yes,
            ident: IdentNode {
              raw: "bar".to_owned()
            },
            ty: TypeKind::Path(
              ImmutablityKind::Yes,
              PathNode {
                ident_list: vec![IdentNode {
                  raw: "Bar".to_owned()
                }]
              }
            )
          }],
          return_type: None,
          block: Some(BlockNode {
            statement_list: vec![StatementKind::Item(Box::new(ItemNode {
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
                      raw: "bool".to_owned(),
                    }]
                  }
                ),
                value: ExpressionKind::Literal(LiteralKind::Bool(true)),
              })
            }))]
          }),
        })]
      }
    );
  }

  #[test]
  fn two_argument_a_body() {
    let source = "trait Foo {
      fn foo(bar: Bar, baz: Baz) {
        const FOO: bool = true;
      }
    }";
    assert_eq!(
      compile(source),
      TraitNode {
        visibility: None,
        ident: IdentNode {
          raw: "Foo".to_owned()
        },
        generic: None,
        inheritances: vec![],
        item_list: vec![TraitItemKind::Function(TraitItemFunctionNode {
          is_async: false,
          ident: IdentNode {
            raw: "foo".to_owned()
          },
          generic: None,
          self_type: None,
          argument_list: vec![
            FunctionArgumentNode {
              immutablity: ImmutablityKind::Yes,
              ident: IdentNode {
                raw: "bar".to_owned()
              },
              ty: TypeKind::Path(
                ImmutablityKind::Yes,
                PathNode {
                  ident_list: vec![IdentNode {
                    raw: "Bar".to_owned()
                  }]
                }
              )
            },
            FunctionArgumentNode {
              immutablity: ImmutablityKind::Yes,
              ident: IdentNode {
                raw: "baz".to_owned()
              },
              ty: TypeKind::Path(
                ImmutablityKind::Yes,
                PathNode {
                  ident_list: vec![IdentNode {
                    raw: "Baz".to_owned()
                  }]
                }
              )
            }
          ],
          return_type: None,
          block: Some(BlockNode {
            statement_list: vec![StatementKind::Item(Box::new(ItemNode {
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
                      raw: "bool".to_owned(),
                    }]
                  }
                ),
                value: ExpressionKind::Literal(LiteralKind::Bool(true)),
              })
            }))]
          }),
        })]
      }
    );
  }
}
