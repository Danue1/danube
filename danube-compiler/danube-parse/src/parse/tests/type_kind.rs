use crate::Parse;
use danube_ast::{IdentNode, PathNode, TypeKind};
use danube_lex::Lex;
use danube_token::{Symbol, Token};

#[test]
fn one() {
    let source = "foo";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_type_kind(),
        Ok(TypeKind::Path(PathNode {
            segments: vec![IdentNode {
                symbol: Symbol::intern("foo"),
            }],
        })),
    );
}

#[test]
fn two() {
    let source = "foo::bar";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_type_kind(),
        Ok(TypeKind::Path(PathNode {
            segments: vec![
                IdentNode {
                    symbol: Symbol::intern("foo"),
                },
                IdentNode {
                    symbol: Symbol::intern("bar"),
                },
            ],
        })),
    );
}

#[test]
fn tuple_with_one() {
    let source = "(foo)";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_type_kind(),
        Ok(TypeKind::Tuple(vec![TypeKind::Path(PathNode {
            segments: vec![IdentNode {
                symbol: Symbol::intern("foo"),
            }],
        })])),
    );
}

#[test]
fn tuple_with_two() {
    let source = "(foo, bar)";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_type_kind(),
        Ok(TypeKind::Tuple(vec![
            TypeKind::Path(PathNode {
                segments: vec![IdentNode {
                    symbol: Symbol::intern("foo"),
                }],
            }),
            TypeKind::Path(PathNode {
                segments: vec![IdentNode {
                    symbol: Symbol::intern("bar"),
                }],
            }),
        ])),
    );
}

#[test]
fn generic_with_one() {
    let source = "foo<bar>";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_type_kind(),
        Ok(TypeKind::Generic(
            PathNode {
                segments: vec![IdentNode {
                    symbol: Symbol::intern("foo"),
                }],
            },
            vec![TypeKind::Path(PathNode {
                segments: vec![IdentNode {
                    symbol: Symbol::intern("bar"),
                }],
            })],
        )),
    );
}

#[test]
fn generic_with_two() {
    let source = "foo<bar, baz>";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_type_kind(),
        Ok(TypeKind::Generic(
            PathNode {
                segments: vec![IdentNode {
                    symbol: Symbol::intern("foo"),
                }],
            },
            vec![
                TypeKind::Path(PathNode {
                    segments: vec![IdentNode {
                        symbol: Symbol::intern("bar"),
                    }],
                }),
                TypeKind::Path(PathNode {
                    segments: vec![IdentNode {
                        symbol: Symbol::intern("baz"),
                    }],
                }),
            ],
        )),
    );
}

#[test]
fn tuple_with_generic_with_one() {
    let source = "(foo<bar>)";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_type_kind(),
        Ok(TypeKind::Tuple(vec![TypeKind::Generic(
            PathNode {
                segments: vec![IdentNode {
                    symbol: Symbol::intern("foo"),
                }],
            },
            vec![TypeKind::Path(PathNode {
                segments: vec![IdentNode {
                    symbol: Symbol::intern("bar"),
                }],
            })],
        )])),
    );
}

#[test]
fn tuple_with_generic_with_two() {
    let source = "(foo<bar, baz>)";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_type_kind(),
        Ok(TypeKind::Tuple(vec![TypeKind::Generic(
            PathNode {
                segments: vec![IdentNode {
                    symbol: Symbol::intern("foo"),
                }],
            },
            vec![
                TypeKind::Path(PathNode {
                    segments: vec![IdentNode {
                        symbol: Symbol::intern("bar"),
                    }],
                }),
                TypeKind::Path(PathNode {
                    segments: vec![IdentNode {
                        symbol: Symbol::intern("baz"),
                    }],
                }),
            ],
        )])),
    );
}

#[test]
fn generic_with_tuple_with_one() {
    let source = "foo<(bar)>";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_type_kind(),
        Ok(TypeKind::Generic(
            PathNode {
                segments: vec![IdentNode {
                    symbol: Symbol::intern("foo"),
                }],
            },
            vec![TypeKind::Tuple(vec![TypeKind::Path(PathNode {
                segments: vec![IdentNode {
                    symbol: Symbol::intern("bar"),
                }],
            })])],
        )),
    );
}

#[test]
fn generic_with_tuple_with_two() {
    let source = "foo<(bar, baz)>";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_type_kind(),
        Ok(TypeKind::Generic(
            PathNode {
                segments: vec![IdentNode {
                    symbol: Symbol::intern("foo"),
                }],
            },
            vec![TypeKind::Tuple(vec![
                TypeKind::Path(PathNode {
                    segments: vec![IdentNode {
                        symbol: Symbol::intern("bar"),
                    }],
                }),
                TypeKind::Path(PathNode {
                    segments: vec![IdentNode {
                        symbol: Symbol::intern("baz"),
                    }],
                }),
            ])],
        )),
    );
}
