use danube_ast::{
    BlockNode, ConditionBranch, ConditionNode, ExpressionKind, ExpressionNode, ForNode, IdentNode,
    LoopNode, MatchBranch, MatchNode, PathNode, PatternKind, PatternNode, WhileNode, DUMMY_NODE_ID,
};
use danube_token::{LiteralKind, Symbol};

assert_node! {
    #[test]
    fn ident() -> ExpressionNode {
        let source = "foo";

        assert_eq!(
            source,
            Ok(ExpressionNode {
                id: DUMMY_NODE_ID,
                kind: ExpressionKind::Path(PathNode {
                    segments: vec![IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("foo"),
                    }],
                }),
            }),
        );
    }

    #[test]
    fn path() -> ExpressionNode {
        let source = "foo::bar";

        assert_eq!(
            source,
            Ok(ExpressionNode {
                id: DUMMY_NODE_ID,
                kind: ExpressionKind::Path(PathNode {
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
    fn add() -> ExpressionNode {
        let source = "+foo";

        assert_eq!(
            source,
            Ok(ExpressionNode {
                id: DUMMY_NODE_ID,
                kind: ExpressionKind::Path(PathNode {
                    segments: vec![IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("foo"),
                    }],
                }),
            }),
        );
    }

    #[test]
    fn negate() -> ExpressionNode {
        let source = "-foo";

        assert_eq!(
            source,
            Ok(ExpressionNode {
                id: DUMMY_NODE_ID,
                kind: ExpressionKind::Negate(Box::new(ExpressionNode {
                    id: DUMMY_NODE_ID,
                    kind: ExpressionKind::Path(PathNode {
                        segments: vec![IdentNode {
                            id: DUMMY_NODE_ID,
                            symbol: Symbol::intern("foo"),
                        }],
                    }),
                })),
            }),
        );
    }

    #[test]
    fn not() -> ExpressionNode {
        let source = "!foo";

        assert_eq!(
            source,
            Ok(ExpressionNode {
                id: DUMMY_NODE_ID,
                kind: ExpressionKind::Not(Box::new(ExpressionNode {
                    id: DUMMY_NODE_ID,
                    kind: ExpressionKind::Path(PathNode {
                        segments: vec![IdentNode {
                            id: DUMMY_NODE_ID,
                            symbol: Symbol::intern("foo"),
                        }],
                    }),
                })),
            }),
        );
    }

    #[test]
    fn char() -> ExpressionNode {
        let source = "'a'";

        assert_eq!(
            source,
            Ok(ExpressionNode {
                id: DUMMY_NODE_ID,
                kind: ExpressionKind::Literal(Symbol::intern("a"), LiteralKind::Char),
            }),
        );
    }

    #[test]
    fn integer() -> ExpressionNode {
        let source = "123";

        assert_eq!(
            source,
            Ok(ExpressionNode {
                id: DUMMY_NODE_ID,
                kind: ExpressionKind::Literal(Symbol::intern("123"), LiteralKind::Integer),
            }),
        );
    }

    #[test]
    fn float() -> ExpressionNode {
        let source = "123.456";

        assert_eq!(
            source,
            Ok(ExpressionNode {
                id: DUMMY_NODE_ID,
                kind: ExpressionKind::Literal(Symbol::intern("123.456"), LiteralKind::Float),
            }),
        );
    }

    #[test]
    fn string() -> ExpressionNode {
        let source = "\"foo\"";

        assert_eq!(
            source,
            Ok(ExpressionNode {
                id: DUMMY_NODE_ID,
                kind: ExpressionKind::Literal(Symbol::intern("foo"), LiteralKind::String),
            }),
        );
    }

    #[test]
    fn conditional_without_else() -> ExpressionNode {
        let source = "if hello { }";

        assert_eq!(
            source,
            Ok(ExpressionNode {
                id: DUMMY_NODE_ID,
                kind: ExpressionKind::Conditional(ConditionNode {
                    branches: vec![ConditionBranch {
                        expression: Box::new(ExpressionNode {
                            id: DUMMY_NODE_ID,
                            kind: ExpressionKind::Path(PathNode {
                                segments: vec![IdentNode {
                                    id: DUMMY_NODE_ID,
                                    symbol: Symbol::intern("hello"),
                                }],
                            }),
                        }),
                        block: BlockNode {
                            id: DUMMY_NODE_ID,
                            statements: vec![]
                        },
                    }],
                    other: None,
                }),
            }),
        );
    }

    #[test]
    fn conditional_with_else() -> ExpressionNode {
        let source = "if hello { } else { }";

        assert_eq!(
            source,
            Ok(ExpressionNode {
                id: DUMMY_NODE_ID,
                kind: ExpressionKind::Conditional(ConditionNode {
                    branches: vec![ConditionBranch {
                        expression: Box::new(ExpressionNode {
                            id: DUMMY_NODE_ID,
                            kind: ExpressionKind::Path(PathNode {
                                segments: vec![IdentNode {
                                    id: DUMMY_NODE_ID,
                                    symbol: Symbol::intern("hello"),
                                }],
                            }),
                        }),
                        block: BlockNode {
                            id: DUMMY_NODE_ID,
                            statements: vec![]
                        },
                    }],
                    other: Some(BlockNode {
                        id: DUMMY_NODE_ID,
                        statements: vec![]
                    }),
                }),
            }),
        );
    }

    #[test]
    fn r#loop() -> ExpressionNode {
        let source = "loop { }";

        assert_eq!(
            source,
            Ok(ExpressionNode {
                id: DUMMY_NODE_ID,
                kind: ExpressionKind::Loop(LoopNode {
                    block: BlockNode {
                        id: DUMMY_NODE_ID,
                        statements: vec![]
                    },
                }),
            }),
        );
    }

    #[test]
    fn r#while() -> ExpressionNode {
        let source = "while hello { }";

        assert_eq!(
            source,
            Ok(ExpressionNode {
                id: DUMMY_NODE_ID,
                kind: ExpressionKind::While(WhileNode {
                    branch: ConditionBranch {
                        expression: Box::new(ExpressionNode {
                            id: DUMMY_NODE_ID,
                            kind: ExpressionKind::Path(PathNode {
                                segments: vec![IdentNode {
                                    id: DUMMY_NODE_ID,
                                    symbol: Symbol::intern("hello"),
                                }],
                            }),
                        }),
                        block: BlockNode {
                            id: DUMMY_NODE_ID,
                            statements: vec![]
                        },
                    },
                }),
            }),
        );
    }

    #[test]
    fn r#for() -> ExpressionNode {
        let source = "for foo in bar { }";

        assert_eq!(
            source,
            Ok(ExpressionNode {
                id: DUMMY_NODE_ID,
                kind: ExpressionKind::For(ForNode {
                    pattern: PatternNode {
                        id: DUMMY_NODE_ID,
                        kind: PatternKind::Path(PathNode {
                            segments: vec![IdentNode {
                                id: DUMMY_NODE_ID,
                                symbol: Symbol::intern("foo"),
                            }],
                        }),
                    },
                    iter: Box::new(ExpressionNode {
                        id: DUMMY_NODE_ID,
                        kind: ExpressionKind::Path(PathNode {
                            segments: vec![IdentNode {
                                id: DUMMY_NODE_ID,
                                symbol: Symbol::intern("bar"),
                            }],
                        }),
                    }),
                    block: BlockNode {
                        id: DUMMY_NODE_ID,
                        statements: vec![]
                    },
                }),
            }),
        );
    }

    #[test]
    fn r#match() -> ExpressionNode {
        let source = "match foo { 1 => { } }";

        assert_eq!(
            source,
            Ok(ExpressionNode {
                id: DUMMY_NODE_ID,
                kind: ExpressionKind::Match(MatchNode {
                    expression: Box::new(ExpressionNode {
                        id: DUMMY_NODE_ID,
                        kind: ExpressionKind::Path(PathNode {
                            segments: vec![IdentNode {
                                id: DUMMY_NODE_ID,
                                symbol: Symbol::intern("foo"),
                            }],
                        }),
                    }),
                    branches: vec![MatchBranch {
                        pattern: PatternNode {
                            id: DUMMY_NODE_ID,
                            kind: PatternKind::Literal(Symbol::intern("1"), LiteralKind::Integer,),
                        },
                        block: BlockNode {
                            id: DUMMY_NODE_ID,
                            statements: vec![]
                        },
                    }],
                }),
            }),
        );
    }
}
