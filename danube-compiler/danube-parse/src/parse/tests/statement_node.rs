use danube_ast::{
    AssignKind, AssignNode, BinaryExpressionNode, BinaryOperatorKind, ExpressionKind,
    ExpressionNode, IdentNode, ImmutabilityKind, LetNode, PathNode, PatternKind, PatternNode,
    StatementKind, StatementNode, TypeKind, TypeNode, DUMMY_NODE_ID,
};
use danube_token::Symbol;

assert_node! {
    #[test]
    fn semicolon() -> StatementNode {
        let source = ";";

        assert_eq!(
            source,
            Ok(StatementNode {
                id: DUMMY_NODE_ID,
                kind: StatementKind::Semicolon,
            }),
        );
    }

    #[test]
    fn r#break() -> StatementNode {
        let source = "break";

        assert_eq!(
            source,
            Ok(StatementNode {
                id: DUMMY_NODE_ID,
                kind: StatementKind::Break,
            }),
        );
    }

    #[test]
    fn r#continue() -> StatementNode {
        let source = "continue";

        assert_eq!(
            source,
            Ok(StatementNode {
                id: DUMMY_NODE_ID,
                kind: StatementKind::Continue,
            }),
        );
    }

    #[test]
    fn return_without_expression() -> StatementNode {
        let source = "return;";

        assert_eq!(
            source,
            Ok(StatementNode {
                id: DUMMY_NODE_ID,
                kind: StatementKind::Return(None),
            }),
        );
    }

    #[test]
    fn return_with_expression() -> StatementNode {
        let source = "return hello;";

        assert_eq!(
            source,
            Ok(StatementNode {
                id: DUMMY_NODE_ID,
                kind: StatementKind::Return(Some(ExpressionNode {
                    id: DUMMY_NODE_ID,
                    kind: ExpressionKind::Path(PathNode {
                        segments: vec![IdentNode {
                            id: DUMMY_NODE_ID,
                            symbol: Symbol::intern("hello"),
                        }],
                    }),
                })),
            }),
        );
    }

    #[test]
    fn return_with_expressions() -> StatementNode {
        let source = "return hello + my + world;";

        assert_eq!(
            source,
            Ok(StatementNode {
                id: DUMMY_NODE_ID,
                kind: StatementKind::Return(Some(ExpressionNode {
                    id: DUMMY_NODE_ID,
                    kind: ExpressionKind::Binary(BinaryExpressionNode {
                        kind: BinaryOperatorKind::Add,
                        lhs: Box::new(ExpressionNode {
                            id: DUMMY_NODE_ID,
                            kind: ExpressionKind::Binary(BinaryExpressionNode {
                                kind: BinaryOperatorKind::Add,
                                lhs: Box::new(ExpressionNode {
                                    id: DUMMY_NODE_ID,
                                    kind: ExpressionKind::Path(PathNode {
                                        segments: vec![IdentNode {
                                            id: DUMMY_NODE_ID,
                                            symbol: Symbol::intern("hello"),
                                        }],
                                    }),
                                }),
                                rhs: Box::new(ExpressionNode {
                                    id: DUMMY_NODE_ID,
                                    kind: ExpressionKind::Path(PathNode {
                                        segments: vec![IdentNode {
                                            id: DUMMY_NODE_ID,
                                            symbol: Symbol::intern("my"),
                                        }],
                                    }),
                                }),
                            }),
                        }),
                        rhs: Box::new(ExpressionNode {
                            id: DUMMY_NODE_ID,
                            kind: ExpressionKind::Path(PathNode {
                                segments: vec![IdentNode {
                                    id: DUMMY_NODE_ID,
                                    symbol: Symbol::intern("world"),
                                }],
                            }),
                        }),
                    }),
                })),
            }),
        );
    }

    #[test]
    fn let_with_nothing() -> StatementNode {
        let source = "let foo;";

        assert_eq!(
            source,
            Ok(StatementNode {
                id: DUMMY_NODE_ID,
                kind: StatementKind::Let(Box::new(LetNode {
                    id: DUMMY_NODE_ID,
                    immutability: ImmutabilityKind::Yes,
                    pattern: PatternNode {
                        id: DUMMY_NODE_ID,
                        kind: PatternKind::Path(PathNode {
                            segments: vec![IdentNode {
                                id: DUMMY_NODE_ID,
                                symbol: Symbol::intern("foo"),
                            }],
                        }),
                    },
                    ty: None,
                    value: None,
                })),
            }),
        );
    }

    #[test]
    fn let_with_type() -> StatementNode {
        let source = "let foo: Foo;";

        assert_eq!(
            source,
            Ok(StatementNode {
                id: DUMMY_NODE_ID,
                kind: StatementKind::Let(Box::new(LetNode {
                    id: DUMMY_NODE_ID,
                    immutability: ImmutabilityKind::Yes,
                    pattern: PatternNode {
                        id: DUMMY_NODE_ID,
                        kind: PatternKind::Path(PathNode {
                            segments: vec![IdentNode {
                                id: DUMMY_NODE_ID,
                                symbol: Symbol::intern("foo"),
                            }],
                        }),
                    },
                    ty: Some(TypeNode {
                        id: DUMMY_NODE_ID,
                        immutability: ImmutabilityKind::Yes,
                        kind: TypeKind::Path(PathNode {
                            segments: vec![IdentNode {
                                id: DUMMY_NODE_ID,
                                symbol: Symbol::intern("Foo"),
                            }],
                        }),
                    }),
                    value: None,
                })),
            }),
        );
    }

    #[test]
    fn let_with_value() -> StatementNode {
        let source = "let foo = bar;";

        assert_eq!(
            source,
            Ok(StatementNode {
                id: DUMMY_NODE_ID,
                kind: StatementKind::Let(Box::new(LetNode {
                    id: DUMMY_NODE_ID,
                    immutability: ImmutabilityKind::Yes,
                    pattern: PatternNode {
                        id: DUMMY_NODE_ID,
                        kind: PatternKind::Path(PathNode {
                            segments: vec![IdentNode {
                                id: DUMMY_NODE_ID,
                                symbol: Symbol::intern("foo"),
                            }],
                        }),
                    },
                    ty: None,
                    value: Some(ExpressionNode {
                        id: DUMMY_NODE_ID,
                        kind: ExpressionKind::Path(PathNode {
                            segments: vec![IdentNode {
                                id: DUMMY_NODE_ID,
                                symbol: Symbol::intern("bar"),
                            }],
                        }),
                    }),
                })),
            }),
        );
    }

    #[test]
    fn assign() -> StatementNode {
        let source = "foo = bar;";

        assert_eq!(
            source,
            Ok(StatementNode {
                id: DUMMY_NODE_ID,
                kind: StatementKind::Assign(Box::new(AssignNode {
                    kind: AssignKind::Assign,
                    lhs: ExpressionNode {
                        id: DUMMY_NODE_ID,
                        kind: ExpressionKind::Path(PathNode {
                            segments: vec![IdentNode {
                                id: DUMMY_NODE_ID,
                                symbol: Symbol::intern("foo"),
                            }],
                        }),
                    },
                    rhs: ExpressionNode {
                        id: DUMMY_NODE_ID,
                        kind: ExpressionKind::Path(PathNode {
                            segments: vec![IdentNode {
                                id: DUMMY_NODE_ID,
                                symbol: Symbol::intern("bar"),
                            }],
                        }),
                    },
                })),
            }),
        );
    }

    #[test]
    fn add_assign() -> StatementNode {
        let source = "foo += bar;";

        assert_eq!(
            source,
            Ok(StatementNode {
                id: DUMMY_NODE_ID,
                kind: StatementKind::Assign(Box::new(AssignNode {
                    kind: AssignKind::Add,
                    lhs: ExpressionNode {
                        id: DUMMY_NODE_ID,
                        kind: ExpressionKind::Path(PathNode {
                            segments: vec![IdentNode {
                                id: DUMMY_NODE_ID,
                                symbol: Symbol::intern("foo"),
                            }],
                        }),
                    },
                    rhs: ExpressionNode {
                        id: DUMMY_NODE_ID,
                        kind: ExpressionKind::Path(PathNode {
                            segments: vec![IdentNode {
                                id: DUMMY_NODE_ID,
                                symbol: Symbol::intern("bar"),
                            }],
                        }),
                    },
                })),
            }),
        );
    }
}
