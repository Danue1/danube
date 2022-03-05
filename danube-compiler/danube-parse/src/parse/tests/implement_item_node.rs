use danube_ast::{
    BlockNode, ConstantNode, ExpressionKind, ExpressionNode, FunctionNode, IdentNode,
    ImmutabilityKind, ImplementItemKind, ImplementItemNode, PathNode, PatternKind, PatternNode,
    TypeAliasNode, TypeKind, TypeNode, DUMMY_NODE_ID,
};
use danube_token::Symbol;

assert_node! {
    #[test]
    fn type_without_type() -> ImplementItemNode {
        let source = "type Foo;";

        assert_eq!(
            source,
            Ok(ImplementItemNode {
                id: DUMMY_NODE_ID,
                attributes: vec![],
                kind: ImplementItemKind::Type(TypeAliasNode {
                    id: DUMMY_NODE_ID,
                    ident: IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("Foo"),
                    },
                    ty: None,
                }),
            }),
        );
    }

    #[test]
    fn type_with_type() -> ImplementItemNode {
        let source = "type Foo = Bar;";

        assert_eq!(
            source,
            Ok(ImplementItemNode {
                id: DUMMY_NODE_ID,
                attributes: vec![],
                kind: ImplementItemKind::Type(TypeAliasNode {
                    id: DUMMY_NODE_ID,
                    ident: IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("Foo"),
                    },
                    ty: Some(TypeNode {
                        id: DUMMY_NODE_ID,
                        immutability: ImmutabilityKind::Yes,
                        kind: TypeKind::Path(PathNode {
                            segments: vec![IdentNode {
                                id: DUMMY_NODE_ID,
                                symbol: Symbol::intern("Bar"),
                            }],
                        }),
                    }),
                }),
            }),
        );
    }

    #[test]
    fn constant_without_value() -> ImplementItemNode {
        let source = "const FOO: Foo;";

        assert_eq!(
            source,
            Ok(ImplementItemNode {
                id: DUMMY_NODE_ID,
                attributes: vec![],
                kind: ImplementItemKind::Constant(ConstantNode {
                    id: DUMMY_NODE_ID,
                    pattern: PatternNode {
                        id: DUMMY_NODE_ID,
                        kind: PatternKind::Path(PathNode {
                            segments: vec![IdentNode {
                                id: DUMMY_NODE_ID,
                                symbol: Symbol::intern("FOO"),
                            }],
                        }),
                    },
                    ty: TypeNode {
                        id: DUMMY_NODE_ID,
                        immutability: ImmutabilityKind::Yes,
                        kind: TypeKind::Path(PathNode {
                            segments: vec![IdentNode {
                                id: DUMMY_NODE_ID,
                                symbol: Symbol::intern("Foo"),
                            }],
                        }),
                    },
                    expression: None,
                }),
            }),
        );
    }

    #[test]
    fn constant_with_value() -> ImplementItemNode {
        let source = "const FOO: Foo = foo;";

        assert_eq!(
            source,
            Ok(ImplementItemNode {
                id: DUMMY_NODE_ID,
                attributes: vec![],
                kind: ImplementItemKind::Constant(ConstantNode {
                    id: DUMMY_NODE_ID,
                    pattern: PatternNode {
                        id: DUMMY_NODE_ID,
                        kind: PatternKind::Path(PathNode {
                            segments: vec![IdentNode {
                                id: DUMMY_NODE_ID,
                                symbol: Symbol::intern("FOO"),
                            }],
                        }),
                    },
                    ty: TypeNode {
                        id: DUMMY_NODE_ID,
                        immutability: ImmutabilityKind::Yes,
                        kind: TypeKind::Path(PathNode {
                            segments: vec![IdentNode {
                                id: DUMMY_NODE_ID,
                                symbol: Symbol::intern("Foo"),
                            }],
                        }),
                    },
                    expression: Some(ExpressionNode {
                        id: DUMMY_NODE_ID,
                        kind: ExpressionKind::Path(PathNode {
                            segments: vec![IdentNode {
                                id: DUMMY_NODE_ID,
                                symbol: Symbol::intern("foo"),
                            }],
                        })
                    }),
                }),
            }),
        );
    }

    #[test]
    fn function_without_body() -> ImplementItemNode {
        let source = "fn foo();";

        assert_eq!(
            source,
            Ok(ImplementItemNode {
                id: DUMMY_NODE_ID,
                attributes: vec![],
                kind: ImplementItemKind::Function(FunctionNode {
                    id: DUMMY_NODE_ID,
                    ident: IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("foo"),
                    },
                    generics: vec![],
                    self_type: None,
                    parameters: vec![],
                    return_type: None,
                    block: None,
                }),
            }),
        );
    }

    #[test]
    fn function_with_body() -> ImplementItemNode {
        let source = "fn foo() {}";

        assert_eq!(
            source,
            Ok(ImplementItemNode {
                id: DUMMY_NODE_ID,
                attributes: vec![],
                kind: ImplementItemKind::Function(FunctionNode {
                    id: DUMMY_NODE_ID,
                    ident: IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("foo"),
                    },
                    generics: vec![],
                    self_type: None,
                    parameters: vec![],
                    return_type: None,
                    block: Some(BlockNode {
                        id: DUMMY_NODE_ID,
                        statements: vec![]
                    }),
                }),
            }),
        );
    }
}
