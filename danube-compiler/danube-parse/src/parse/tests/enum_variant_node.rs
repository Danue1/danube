use danube_ast::{
    EnumNamedVariantNode, EnumVariantKind, EnumVariantNode, IdentNode, ImmutabilityKind, PathNode,
    TypeKind, TypeNode, DUMMY_NODE_ID,
};
use danube_token::Symbol;

assert_node! {
    #[test]
    fn without_struct() -> EnumVariantNode {
        let source = "Foo";

        assert_eq!(
            source,
            Ok(EnumVariantNode {
                id: DUMMY_NODE_ID,
                ident: IdentNode {
                    id: DUMMY_NODE_ID,
                    symbol: Symbol::intern("Foo")
                },
                kind: None,
            }),
        );
    }


    #[test]
    fn unnamed_without_fields() -> EnumVariantNode {
        let source = "Foo()";

        assert_eq!(source,
            Ok(EnumVariantNode {
                id: DUMMY_NODE_ID,
                ident: IdentNode {
                    id: DUMMY_NODE_ID,
                    symbol: Symbol::intern("Foo")
                },
                kind: Some(EnumVariantKind::Unnamed(vec![])),
            }),
        );
    }


    #[test]
    fn unnamed_with_one_field() -> EnumVariantNode {
        let source = "Foo(Bar)";

        assert_eq!(source,
            Ok(EnumVariantNode {
                id: DUMMY_NODE_ID,
                ident: IdentNode {
                    id: DUMMY_NODE_ID,
                    symbol: Symbol::intern("Foo")
                },
                kind: Some(EnumVariantKind::Unnamed(vec![TypeNode {
                    id: DUMMY_NODE_ID,
                    immutability: ImmutabilityKind::Yes,
                    kind: TypeKind::Path(PathNode {
                        segments: vec![IdentNode {
                            id: DUMMY_NODE_ID,
                            symbol: Symbol::intern("Bar"),
                        }],
                    }),
                }])),
            }),
        );
    }


    #[test]
    fn unnamed_with_two_field() -> EnumVariantNode {
        let source = "Foo(Bar, Baz)";

        assert_eq!(source,
            Ok(EnumVariantNode {
                id: DUMMY_NODE_ID,
                ident: IdentNode {
                    id: DUMMY_NODE_ID,
                    symbol: Symbol::intern("Foo")
                },
                kind: Some(EnumVariantKind::Unnamed(vec![
                    TypeNode {
                        id: DUMMY_NODE_ID,
                        immutability: ImmutabilityKind::Yes,
                        kind: TypeKind::Path(PathNode {
                            segments: vec![IdentNode {
                                id: DUMMY_NODE_ID,
                                symbol: Symbol::intern("Bar"),
                            }],
                        }),
                    },
                    TypeNode {
                        id: DUMMY_NODE_ID,
                        immutability: ImmutabilityKind::Yes,
                        kind: TypeKind::Path(PathNode {
                            segments: vec![IdentNode {
                                id: DUMMY_NODE_ID,
                                symbol: Symbol::intern("Baz"),
                            }],
                        }),
                    },
                ])),
            }),
        );
    }


    #[test]
    fn named_with_one_field() -> EnumVariantNode {
        let source = "Foo { bar: Bar }";

        assert_eq!(source,
            Ok(EnumVariantNode {
                id: DUMMY_NODE_ID,
                ident: IdentNode {
                    id: DUMMY_NODE_ID,
                    symbol: Symbol::intern("Foo")
                },
                kind: Some(EnumVariantKind::Named(vec![EnumNamedVariantNode {
                    ident: IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("bar"),
                    },
                    ty: TypeNode {
                        id: DUMMY_NODE_ID,
                        immutability: ImmutabilityKind::Yes,
                        kind: TypeKind::Path(PathNode {
                            segments: vec![IdentNode {
                                id: DUMMY_NODE_ID,
                                symbol: Symbol::intern("Bar"),
                            }],
                        }),
                    },
                }])),
            }),
        );
    }

    #[test]
    fn named_with_two_field() -> EnumVariantNode {
        let source = "Foo { bar: Bar, baz: Baz }";

        assert_eq!(source,
            Ok(EnumVariantNode {
                id: DUMMY_NODE_ID,
                ident: IdentNode {
                    id: DUMMY_NODE_ID,
                    symbol: Symbol::intern("Foo")
                },
                kind: Some(EnumVariantKind::Named(vec![
                    EnumNamedVariantNode {
                        ident: IdentNode {
                            id: DUMMY_NODE_ID,
                            symbol: Symbol::intern("bar"),
                        },
                        ty: TypeNode {
                            id: DUMMY_NODE_ID,
                            immutability: ImmutabilityKind::Yes,
                            kind: TypeKind::Path(PathNode {
                                segments: vec![IdentNode {
                                    id: DUMMY_NODE_ID,
                                    symbol: Symbol::intern("Bar"),
                                }],
                            }),
                        },
                    },
                    EnumNamedVariantNode {
                        ident: IdentNode {
                            id: DUMMY_NODE_ID,
                            symbol: Symbol::intern("baz"),
                        },
                        ty: TypeNode {
                            id: DUMMY_NODE_ID,
                            immutability: ImmutabilityKind::Yes,
                            kind: TypeKind::Path(PathNode {
                                segments: vec![IdentNode {
                                    id: DUMMY_NODE_ID,
                                    symbol: Symbol::intern("Baz"),
                                }],
                            }),
                        },
                    },
                ])),
            }),
        );
    }
}
