use danube_ast::{
    BlockNode, FunctionNode, FunctionParameterNode, IdentNode, ImmutabilityKind, PathNode,
    TypeKind, TypeNode, DUMMY_NODE_ID,
};
use danube_token::Symbol;

assert_node! {
    #[test]
    fn without_block() -> FunctionNode {
        let source = "foo();";

        assert_eq!(
            source,
            Ok(FunctionNode {
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
        );
    }

    #[test]
    fn with_block() -> FunctionNode {
        let source = "foo() { }";

        assert_eq!(
            source,
            Ok(FunctionNode {
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
        );
    }

    #[test]
    fn with_return_type() -> FunctionNode {
        let source = "foo() -> bar;";

        assert_eq!(
            source,
            Ok(FunctionNode {
                ident: IdentNode {
                    id: DUMMY_NODE_ID,
                    symbol: Symbol::intern("foo"),
                },
                generics: vec![],
                self_type: None,
                parameters: vec![],
                return_type: Some(TypeNode {
                    id: DUMMY_NODE_ID,
                    immutability: ImmutabilityKind::Yes,
                    kind: TypeKind::Path(PathNode {
                        segments: vec![IdentNode {
                            id: DUMMY_NODE_ID,
                            symbol: Symbol::intern("bar"),
                        }],
                    }),
                }),
                block: None,
            }),
        );
    }

    #[test]
    fn immutable_self() -> FunctionNode {
        let source = "foo(self);";

        assert_eq!(
            source,
            Ok(FunctionNode {
                ident: IdentNode {
                    id: DUMMY_NODE_ID,
                    symbol: Symbol::intern("foo"),
                },
                generics: vec![],
                self_type: Some(ImmutabilityKind::Yes),
                parameters: vec![],
                return_type: None,
                block: None,
            }),
        );
    }

    #[test]
    fn mutable_self() -> FunctionNode {
        let source = "foo(mut self);";

        assert_eq!(
            source,
            Ok(FunctionNode {
                ident: IdentNode {
                    id: DUMMY_NODE_ID,
                    symbol: Symbol::intern("foo"),
                },
                generics: vec![],
                self_type: Some(ImmutabilityKind::Nope),
                parameters: vec![],
                return_type: None,
                block: None,
            }),
        );
    }

    #[test]
    fn one_parameter() -> FunctionNode {
        let source = "foo(bar: Bar);";

        assert_eq!(
            source,
            Ok(FunctionNode {
                ident: IdentNode {
                    id: DUMMY_NODE_ID,
                    symbol: Symbol::intern("foo"),
                },
                generics: vec![],
                self_type: None,
                parameters: vec![FunctionParameterNode {
                    id: DUMMY_NODE_ID,
                    argument_label: IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("bar"),
                    },
                    parameter_label: None,
                    ty: TypeNode {
                        id: DUMMY_NODE_ID,
                        immutability: ImmutabilityKind::Yes,
                        kind: TypeKind::Path(PathNode {
                            segments: vec![IdentNode {
                                id: DUMMY_NODE_ID,
                                symbol: Symbol::intern("Bar"),
                            }],
                        }),
                    }
                }],
                return_type: None,
                block: None,
            }),
        );
    }

    #[test]
    fn two_parameters() -> FunctionNode {
        let source = "foo(bar: Bar, baz: Baz);";

        assert_eq!(
            source,
            Ok(FunctionNode {
                ident: IdentNode {
                    id: DUMMY_NODE_ID,
                    symbol: Symbol::intern("foo"),
                },
                generics: vec![],
                self_type: None,
                parameters: vec![
                    FunctionParameterNode {
                        id: DUMMY_NODE_ID,
                        argument_label: IdentNode {
                            id: DUMMY_NODE_ID,
                            symbol: Symbol::intern("bar"),
                        },
                        parameter_label: None,
                        ty: TypeNode {
                            id: DUMMY_NODE_ID,
                            immutability: ImmutabilityKind::Yes,
                            kind: TypeKind::Path(PathNode {
                                segments: vec![IdentNode {
                                    id: DUMMY_NODE_ID,
                                    symbol: Symbol::intern("Bar"),
                                }],
                            }),
                        }
                    },
                    FunctionParameterNode {
                        id: DUMMY_NODE_ID,
                        argument_label: IdentNode {
                            id: DUMMY_NODE_ID,
                            symbol: Symbol::intern("baz"),
                        },
                        parameter_label: None,
                        ty: TypeNode {
                            id: DUMMY_NODE_ID,
                            immutability: ImmutabilityKind::Yes,
                            kind: TypeKind::Path(PathNode {
                                segments: vec![IdentNode {
                                    id: DUMMY_NODE_ID,
                                    symbol: Symbol::intern("Baz"),
                                }],
                            }),
                        }
                    },
                ],
                return_type: None,
                block: None,
            }),
        );
    }
}
