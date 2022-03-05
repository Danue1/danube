use danube_ast::{GenericNode, IdentNode, ImplementNode, PathNode, DUMMY_NODE_ID};
use danube_token::Symbol;

assert_node! {
    #[test]
    fn with_nothing() -> ImplementNode {
        let source = "Foo {}";

        assert_eq!(
            source,
            Ok(ImplementNode {
                id: DUMMY_NODE_ID,
                generics: vec![],
                trait_ident: None,
                target: PathNode {
                    segments: vec![IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("Foo"),
                    }],
                },
                target_generics: vec![],
                items: vec![],
            }),
        );
    }

    #[test]
    fn with_generics() -> ImplementNode {
        let source = "<T>Foo {}";

        assert_eq!(
            source,
            Ok(ImplementNode {
                id: DUMMY_NODE_ID,
                generics: vec![GenericNode {
                    id: DUMMY_NODE_ID,
                    ident: IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("T"),
                    },
                    traits: vec![],
                    default: None,
                }],
                trait_ident: None,
                target: PathNode {
                    segments: vec![IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("Foo"),
                    }],
                },
                target_generics: vec![],
                items: vec![],
            }),
        );
    }

    #[test]
    fn target_generics_with_generics() -> ImplementNode {
        let source = "<T>Foo<T> {}";

        assert_eq!(
            source,
            Ok(ImplementNode {
                id: DUMMY_NODE_ID,
                generics: vec![GenericNode {
                    id: DUMMY_NODE_ID,
                    ident: IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("T"),
                    },
                    traits: vec![],
                    default: None,
                }],
                trait_ident: None,
                target: PathNode {
                    segments: vec![IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("Foo"),
                    }],
                },
                target_generics: vec![GenericNode {
                    id: DUMMY_NODE_ID,
                    ident: IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("T"),
                    },
                    traits: vec![],
                    default: None,
                }],
                items: vec![],
            }),
        );
    }

    #[test]
    fn for_with_generics() -> ImplementNode {
        let source = "Foo for Bar {}";

        assert_eq!(
            source,
            Ok(ImplementNode {
                id: DUMMY_NODE_ID,
                generics: vec![],
                trait_ident: Some(PathNode {
                    segments: vec![IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("Foo"),
                    }],
                }),
                target: PathNode {
                    segments: vec![IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("Bar"),
                    }],
                },
                target_generics: vec![],
                items: vec![],
            }),
        );
    }

    #[test]
    fn for_generics_with_generics() -> ImplementNode {
        let source = "Foo for Bar<T> {}";

        assert_eq!(
            source,
            Ok(ImplementNode {
                id: DUMMY_NODE_ID,
                generics: vec![],
                trait_ident: Some(PathNode {
                    segments: vec![IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("Foo"),
                    }],
                }),
                target: PathNode {
                    segments: vec![IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("Bar"),
                    }],
                },
                target_generics: vec![GenericNode {
                    id: DUMMY_NODE_ID,
                    ident: IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("T"),
                    },
                    traits: vec![],
                    default: None,
                }],
                items: vec![],
            }),
        );
    }
}
