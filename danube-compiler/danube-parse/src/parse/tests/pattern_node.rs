use crate::Parse;
use danube_ast::{IdentNode, LiteralKind, PathNode, PatternKind, PatternNode};
use danube_lex::Lex;
use danube_token::{Symbol, Token};

#[test]
fn wildcard() {
    let source = "_";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_pattern_node(),
        Ok(PatternNode {
            kind: PatternKind::Wildcard,
        }),
    );
}

#[test]
fn rest() {
    let source = "..";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_pattern_node(),
        Ok(PatternNode {
            kind: PatternKind::Rest,
        }),
    );
}

#[test]
fn literal() {
    let source = "1";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_pattern_node(),
        Ok(PatternNode {
            kind: PatternKind::Literal(Symbol::intern("1"), LiteralKind::Integer),
        }),
    );
}

#[test]
fn path() {
    let source = "foo::bar";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_pattern_node(),
        Ok(PatternNode {
            kind: PatternKind::Path(PathNode {
                idents: vec![
                    IdentNode {
                        symbol: Symbol::intern("foo"),
                    },
                    IdentNode {
                        symbol: Symbol::intern("bar"),
                    },
                ],
            }),
        })
    );
}

#[test]
fn named_struct_with_sugar() {
    let source = "foo { a, b }";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_pattern_node(),
        Ok(PatternNode {
            kind: PatternKind::NamedStruct(
                PathNode {
                    idents: vec![IdentNode {
                        symbol: Symbol::intern("foo"),
                    }],
                },
                vec![
                    (
                        PathNode {
                            idents: vec![IdentNode {
                                symbol: Symbol::intern("a"),
                            }],
                        },
                        None,
                    ),
                    (
                        PathNode {
                            idents: vec![IdentNode {
                                symbol: Symbol::intern("b"),
                            }],
                        },
                        None,
                    ),
                ],
            ),
        })
    );
}

#[test]
fn named_struct_without_sugar() {
    let source = "foo { a: _a, b: _b }";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_pattern_node(),
        Ok(PatternNode {
            kind: PatternKind::NamedStruct(
                PathNode {
                    idents: vec![IdentNode {
                        symbol: Symbol::intern("foo"),
                    }],
                },
                vec![
                    (
                        PathNode {
                            idents: vec![IdentNode {
                                symbol: Symbol::intern("a"),
                            }],
                        },
                        Some(PatternNode {
                            kind: PatternKind::Path(PathNode {
                                idents: vec![IdentNode {
                                    symbol: Symbol::intern("_a"),
                                }],
                            }),
                        }),
                    ),
                    (
                        PathNode {
                            idents: vec![IdentNode {
                                symbol: Symbol::intern("b"),
                            }],
                        },
                        Some(PatternNode {
                            kind: PatternKind::Path(PathNode {
                                idents: vec![IdentNode {
                                    symbol: Symbol::intern("_b"),
                                }],
                            }),
                        }),
                    ),
                ],
            ),
        })
    );
}

#[test]
fn unnamed_struct_with_sugar() {
    let source = "foo(a, b)";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        Parse::new(tokens.as_slice()).parse_pattern_node(),
        Ok(PatternNode {
            kind: PatternKind::UnnamedStruct(
                Some(PathNode {
                    idents: vec![IdentNode {
                        symbol: Symbol::intern("foo"),
                    }],
                }),
                vec![
                    PatternNode {
                        kind: PatternKind::Path(PathNode {
                            idents: vec![IdentNode {
                                symbol: Symbol::intern("a"),
                            }],
                        }),
                    },
                    PatternNode {
                        kind: PatternKind::Path(PathNode {
                            idents: vec![IdentNode {
                                symbol: Symbol::intern("b"),
                            }],
                        }),
                    },
                ],
            ),
        })
    );
}
