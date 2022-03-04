use danube_ast::{
    IdentNode, ImmutabilityKind, NamedStructField, PathNode, StructFieldKind, StructNode, TypeKind,
    TypeNode, UnnamedStructField, VisibilityKind, DUMMY_NODE_ID,
};
use danube_token::Symbol;

assert_node! {
    #[test]
    fn empty() -> StructNode {
        let source = "Foo;";

        assert_eq!(
            source,
            Ok(StructNode {
                ident: IdentNode {
                    id: DUMMY_NODE_ID,
                    symbol: Symbol::intern("Foo"),
                },
                generics: vec![],
                fields: None,
            }),
        );
    }

    #[test]
    fn unnamed_without_fields() -> StructNode {
        let source = "Foo()";

        assert_eq!(
            source,
            Ok(StructNode {
                ident: IdentNode {
                    id: DUMMY_NODE_ID,
                    symbol: Symbol::intern("Foo"),
                },
                generics: vec![],
                fields: Some(StructFieldKind::Unnamed(vec![])),
            }),
        );
    }

    #[test]
    fn unnamed_with_field() -> StructNode {
        let source = "Foo(Bar)";

        assert_eq!(
            source,
            Ok(StructNode {
                ident: IdentNode {
                    id: DUMMY_NODE_ID,
                    symbol: Symbol::intern("Foo"),
                },
                generics: vec![],
                fields: Some(StructFieldKind::Unnamed(vec![
                    UnnamedStructField {
                        visibility: VisibilityKind::Current,
                        ty: TypeNode {
                            id: DUMMY_NODE_ID,
                            immutability: ImmutabilityKind::Yes,
                            kind: TypeKind::Path(PathNode {
                                segments: vec![
                                    IdentNode {
                                        id: DUMMY_NODE_ID,
                                        symbol: Symbol::intern("Bar"),
                                    },
                                ],
                            }),
                        },
                    },
                ])),
            }),
        );
    }

    #[test]
    fn named_without_fields() -> StructNode {
        let source = "Foo { }";

        assert_eq!(
            source,
            Ok(StructNode {
                ident: IdentNode {
                    id: DUMMY_NODE_ID,
                    symbol: Symbol::intern("Foo"),
                },
                generics: vec![],
                fields: Some(StructFieldKind::Named(vec![])),
            }),
        );
    }

    #[test]
    fn named_with_field() -> StructNode {
        let source = "Foo { bar: Bar }";

        assert_eq!(
            source,
            Ok(StructNode {
                ident: IdentNode {
                    id: DUMMY_NODE_ID,
                    symbol: Symbol::intern("Foo"),
                },
                generics: vec![],
                fields: Some(StructFieldKind::Named(vec![
                    NamedStructField {
                        visibility: VisibilityKind::Current,
                        ident: IdentNode {
                            id: DUMMY_NODE_ID,
                            symbol: Symbol::intern("bar"),
                        },
                        ty: TypeNode {
                            id: DUMMY_NODE_ID,
                            immutability: ImmutabilityKind::Yes,
                            kind: TypeKind::Path(PathNode {
                                segments: vec![
                                    IdentNode {
                                        id: DUMMY_NODE_ID,
                                        symbol: Symbol::intern("Bar"),
                                    },
                                ],
                            }),
                        },
                    },
                ])),
            }),
        );
    }
}
