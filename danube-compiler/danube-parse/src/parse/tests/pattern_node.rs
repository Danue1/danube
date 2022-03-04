use danube_ast::{
    IdentNode, LiteralKind, LiteralNode, PathNode, PatternKind, PatternNamedStructNode,
    PatternNode, PatternUnnamedStructNode, DUMMY_NODE_ID,
};
use danube_token::Symbol;

assert_node! {
    #[test]
    fn wildcard() -> PatternNode {
        let source = "_";

        assert_eq!(
            source,
            Ok(PatternNode {
                id: DUMMY_NODE_ID,
                kind: PatternKind::Wildcard,
            }),
        );
    }

    #[test]
    fn rest() -> PatternNode {
        let source = "..";

        assert_eq!(
            source,
            Ok(PatternNode {
                id: DUMMY_NODE_ID,
                kind: PatternKind::Rest,
            }),
        );
    }

    #[test]
    fn literal() -> PatternNode {
        let source = "1";

        assert_eq!(
            source,
            Ok(PatternNode {
                id: DUMMY_NODE_ID,
                kind: PatternKind::Literal(LiteralNode {
                    symbol: Symbol::intern("1"),
                    kind: LiteralKind::Integer,
                }),
            }),
        );
    }

    #[test]
    fn path() -> PatternNode {
        let source = "foo::bar";

        assert_eq!(
            source,
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
            }),
        );
    }

    #[test]
    fn named_struct_with_sugar() -> PatternNode {
        let source = "foo { a, b }";

        assert_eq!(
            source,
            Ok(PatternNode {
                id: DUMMY_NODE_ID,
                kind: PatternKind::NamedStruct(PatternNamedStructNode {
                    path: PathNode {
                        segments: vec![IdentNode {
                            id: DUMMY_NODE_ID,
                            symbol: Symbol::intern("foo"),
                        }],
                    },
                    fields: vec![
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
                }),
            }),
        );
    }

    #[test]
    fn named_struct_without_sugar() -> PatternNode {
        let source = "foo { a: _a, b: _b }";

        assert_eq!(
            source,
            Ok(PatternNode {
                id: DUMMY_NODE_ID,
                kind: PatternKind::NamedStruct(PatternNamedStructNode {
                    path: PathNode {
                        segments: vec![IdentNode {
                            id: DUMMY_NODE_ID,
                            symbol: Symbol::intern("foo"),
                        }],
                    },
                    fields: vec![
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
                }),
            }),
        );
    }

    #[test]
    fn unnamed_struct_with_sugar() -> PatternNode {
        let source = "foo(a, b)";

        assert_eq!(
            source,
            Ok(PatternNode {
                id: DUMMY_NODE_ID,
                kind: PatternKind::UnnamedStruct(PatternUnnamedStructNode {
                    path: Some(PathNode {
                        segments: vec![IdentNode {
                            id: DUMMY_NODE_ID,
                            symbol: Symbol::intern("foo"),
                        }],
                    }),
                    fields: vec![
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
                }),
            }),
        );
    }
}
