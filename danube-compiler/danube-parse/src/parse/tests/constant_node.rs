use danube_ast::{
    ConstantNode, ExpressionKind, ExpressionNode, IdentNode, ImmutabilityKind, PathNode,
    PatternKind, PatternNode, TypeKind, TypeNode, DUMMY_NODE_ID,
};
use danube_token::Symbol;

assert_node! {
    #[test]
    fn without_expression() -> ConstantNode {
        let source = "foo: bar;";

        assert_eq!(
            source,
            Ok(ConstantNode {
                pattern: PatternNode {
                    id: DUMMY_NODE_ID,
                    kind: PatternKind::Path(PathNode {
                        segments: vec![IdentNode {
                            id: DUMMY_NODE_ID,
                            symbol: Symbol::intern("foo"),
                        }],
                    }),
                },
                ty: TypeNode {
                    id: DUMMY_NODE_ID,
                    immutability: ImmutabilityKind::Yes,
                    kind: TypeKind::Path(PathNode {
                        segments: vec![IdentNode {
                            id: DUMMY_NODE_ID,
                            symbol: Symbol::intern("bar"),
                        }],
                    }),
                },
                expression: None,
            }),
        );
    }

    #[test]
    fn with_expression() -> ConstantNode {
        let source = "foo: bar = baz;";

        assert_eq!(
            source,
            Ok(ConstantNode {
                pattern: PatternNode {
                    id: DUMMY_NODE_ID,
                    kind: PatternKind::Path(PathNode {
                        segments: vec![IdentNode {
                            id: DUMMY_NODE_ID,
                            symbol: Symbol::intern("foo"),
                        }],
                    }),
                },
                ty: TypeNode {
                    id: DUMMY_NODE_ID,
                    immutability: ImmutabilityKind::Yes,
                    kind: TypeKind::Path(PathNode {
                        segments: vec![IdentNode {
                            id: DUMMY_NODE_ID,
                            symbol: Symbol::intern("bar"),
                        }],
                    }),
                },
                expression: Some(ExpressionNode {
                    id: DUMMY_NODE_ID,
                    kind: ExpressionKind::Path(PathNode {
                        segments: vec![IdentNode {
                            id: DUMMY_NODE_ID,
                            symbol: Symbol::intern("baz"),
                        }],
                    })
                }),
            }),
        );
    }
}
