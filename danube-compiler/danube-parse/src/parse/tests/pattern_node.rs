use crate::{Context, Parse};
use danube_ast::{IdentNode, LiteralKind, PathNode, PatternKind, PatternNode, DUMMY_NODE_ID};
use danube_lex::Lex;
use danube_token::{Symbol, Token};

#[test]
fn wildcard() {
    let source = "_";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        PatternNode::parse(&mut Context::new(tokens.as_slice())),
        Ok(PatternNode {
            id: DUMMY_NODE_ID,
            kind: PatternKind::Wildcard,
        }),
    );
}

#[test]
fn rest() {
    let source = "..";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        PatternNode::parse(&mut Context::new(tokens.as_slice())),
        Ok(PatternNode {
            id: DUMMY_NODE_ID,
            kind: PatternKind::Rest,
        }),
    );
}

#[test]
fn literal() {
    let source = "1";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        PatternNode::parse(&mut Context::new(tokens.as_slice())),
        Ok(PatternNode {
            id: DUMMY_NODE_ID,
            kind: PatternKind::Literal(Symbol::intern("1"), LiteralKind::Integer),
        }),
    );
}

#[test]
fn path() {
    let source = "foo::bar";
    let tokens: Vec<Token> = Lex::new(source).filter_map(|token| token.ok()).collect();

    assert_eq!(
        PatternNode::parse(&mut Context::new(tokens.as_slice())),
        Ok(PatternNode {
            id: DUMMY_NODE_ID,
            kind: PatternKind::Path(PathNode {
                segments: vec![
                    IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("foo"),
                    },
                    IdentNode {
                        id: DUMMY_NODE_ID,
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
        PatternNode::parse(&mut Context::new(tokens.as_slice())),
        Ok(PatternNode {
            id: DUMMY_NODE_ID,
            kind: PatternKind::NamedStruct(
                PathNode {
                    segments: vec![IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("foo"),
                    }],
                },
                vec![
                    (
                        PathNode {
                            segments: vec![IdentNode {
                                id: DUMMY_NODE_ID,
                                symbol: Symbol::intern("a"),
                            }],
                        },
                        None,
                    ),
                    (
                        PathNode {
                            segments: vec![IdentNode {
                                id: DUMMY_NODE_ID,
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
        PatternNode::parse(&mut Context::new(tokens.as_slice())),
        Ok(PatternNode {
            id: DUMMY_NODE_ID,
            kind: PatternKind::NamedStruct(
                PathNode {
                    segments: vec![IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("foo"),
                    }],
                },
                vec![
                    (
                        PathNode {
                            segments: vec![IdentNode {
                                id: DUMMY_NODE_ID,
                                symbol: Symbol::intern("a"),
                            }],
                        },
                        Some(PatternNode {
                            id: DUMMY_NODE_ID,
                            kind: PatternKind::Path(PathNode {
                                segments: vec![IdentNode {
                                    id: DUMMY_NODE_ID,
                                    symbol: Symbol::intern("_a"),
                                }],
                            }),
                        }),
                    ),
                    (
                        PathNode {
                            segments: vec![IdentNode {
                                id: DUMMY_NODE_ID,
                                symbol: Symbol::intern("b"),
                            }],
                        },
                        Some(PatternNode {
                            id: DUMMY_NODE_ID,
                            kind: PatternKind::Path(PathNode {
                                segments: vec![IdentNode {
                                    id: DUMMY_NODE_ID,
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
        PatternNode::parse(&mut Context::new(tokens.as_slice())),
        Ok(PatternNode {
            id: DUMMY_NODE_ID,
            kind: PatternKind::UnnamedStruct(
                Some(PathNode {
                    segments: vec![IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("foo"),
                    }],
                }),
                vec![
                    PatternNode {
                        id: DUMMY_NODE_ID,
                        kind: PatternKind::Path(PathNode {
                            segments: vec![IdentNode {
                                id: DUMMY_NODE_ID,
                                symbol: Symbol::intern("a"),
                            }],
                        }),
                    },
                    PatternNode {
                        id: DUMMY_NODE_ID,
                        kind: PatternKind::Path(PathNode {
                            segments: vec![IdentNode {
                                id: DUMMY_NODE_ID,
                                symbol: Symbol::intern("b"),
                            }],
                        }),
                    },
                ],
            ),
        })
    );
}
