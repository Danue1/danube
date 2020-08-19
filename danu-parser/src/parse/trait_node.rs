use super::*;

pub(super) fn parse_trait_node(s: Tokens) -> ParseResult<TraitNode> {
  map(
    tuple((
      opt(parse_visibility),
      parse_keyword(Keyword::Trait),
      parse_ident_node,
      opt(parse_generic_node),
      parse_symbol(Symbol::LeftBrace),
      many1(parse_trait_item_node),
      parse_symbol(Symbol::RightBrace),
    )),
    |(visibility, _, ident, generic, _, item_list, _)| TraitNode {
      visibility,
      ident,
      generic,
      item_list,
    },
  )(s)
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
        item_list: vec![TraitItemNode::Constant(TraitItemConstantNode {
          ident: IdentNode {
            raw: "FOO".to_owned()
          },
          ty: TypeNode::Path(PathNode {
            ident_list: vec![IdentNode {
              raw: "Bar".to_owned()
            }]
          }),
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
        item_list: vec![TraitItemNode::Constant(TraitItemConstantNode {
          ident: IdentNode {
            raw: "FOO".to_owned()
          },
          ty: TypeNode::Path(PathNode {
            ident_list: vec![IdentNode {
              raw: "Bar".to_owned()
            }]
          }),
          default_value: Some(LiteralValueNode::Bool(true)),
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
        item_list: vec![TraitItemNode::Function(TraitItemFunctionNode {
          is_async: false,
          ident: IdentNode {
            raw: "foo".to_owned()
          },
          generic: None,
          argument_list: vec![],
          return_type: None,
          body: None,
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
        item_list: vec![TraitItemNode::Function(TraitItemFunctionNode {
          is_async: false,
          ident: IdentNode {
            raw: "foo".to_owned()
          },
          generic: None,
          argument_list: vec![FunctionArgumentNode {
            ident: IdentNode {
              raw: "bar".to_owned()
            },
            ty: TypeNode::Path(PathNode {
              ident_list: vec![IdentNode {
                raw: "Bar".to_owned()
              }]
            })
          }],
          return_type: None,
          body: None,
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
        item_list: vec![TraitItemNode::Function(TraitItemFunctionNode {
          is_async: false,
          ident: IdentNode {
            raw: "foo".to_owned()
          },
          generic: None,
          argument_list: vec![
            FunctionArgumentNode {
              ident: IdentNode {
                raw: "bar".to_owned()
              },
              ty: TypeNode::Path(PathNode {
                ident_list: vec![IdentNode {
                  raw: "Bar".to_owned()
                }]
              })
            },
            FunctionArgumentNode {
              ident: IdentNode {
                raw: "baz".to_owned()
              },
              ty: TypeNode::Path(PathNode {
                ident_list: vec![IdentNode {
                  raw: "Baz".to_owned()
                }]
              })
            }
          ],
          return_type: None,
          body: None,
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
        item_list: vec![TraitItemNode::Function(TraitItemFunctionNode {
          is_async: false,
          ident: IdentNode {
            raw: "foo".to_owned()
          },
          generic: None,
          argument_list: vec![],
          return_type: None,
          body: Some(vec![StatementNode::Constant(ConstantNode {
            visibility: None,
            ident: IdentNode {
              raw: "FOO".to_owned(),
            },
            ty: TypeNode::Path(PathNode {
              ident_list: vec![IdentNode {
                raw: "bool".to_owned(),
              }]
            }),
            value: ExpressionNode::Literal(LiteralValueNode::Bool(true)),
          })]),
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
        item_list: vec![TraitItemNode::Function(TraitItemFunctionNode {
          is_async: false,
          ident: IdentNode {
            raw: "foo".to_owned()
          },
          generic: None,
          argument_list: vec![FunctionArgumentNode {
            ident: IdentNode {
              raw: "bar".to_owned()
            },
            ty: TypeNode::Path(PathNode {
              ident_list: vec![IdentNode {
                raw: "Bar".to_owned()
              }]
            })
          }],
          return_type: None,
          body: Some(vec![StatementNode::Constant(ConstantNode {
            visibility: None,
            ident: IdentNode {
              raw: "FOO".to_owned(),
            },
            ty: TypeNode::Path(PathNode {
              ident_list: vec![IdentNode {
                raw: "bool".to_owned(),
              }]
            }),
            value: ExpressionNode::Literal(LiteralValueNode::Bool(true)),
          })]),
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
        item_list: vec![TraitItemNode::Function(TraitItemFunctionNode {
          is_async: false,
          ident: IdentNode {
            raw: "foo".to_owned()
          },
          generic: None,
          argument_list: vec![
            FunctionArgumentNode {
              ident: IdentNode {
                raw: "bar".to_owned()
              },
              ty: TypeNode::Path(PathNode {
                ident_list: vec![IdentNode {
                  raw: "Bar".to_owned()
                }]
              })
            },
            FunctionArgumentNode {
              ident: IdentNode {
                raw: "baz".to_owned()
              },
              ty: TypeNode::Path(PathNode {
                ident_list: vec![IdentNode {
                  raw: "Baz".to_owned()
                }]
              })
            }
          ],
          return_type: None,
          body: Some(vec![StatementNode::Constant(ConstantNode {
            visibility: None,
            ident: IdentNode {
              raw: "FOO".to_owned(),
            },
            ty: TypeNode::Path(PathNode {
              ident_list: vec![IdentNode {
                raw: "bool".to_owned(),
              }]
            }),
            value: ExpressionNode::Literal(LiteralValueNode::Bool(true)),
          })]),
        })]
      }
    );
  }
}
