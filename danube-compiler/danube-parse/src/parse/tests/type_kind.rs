use danube_ast::{GenericTypeNode, IdentNode, PathNode, TypeKind, DUMMY_NODE_ID};
use danube_token::Symbol;

assert_node! {
    #[test]
    fn one() -> TypeKind {
        let source = "foo";

        assert_eq!(
            source,
            Ok(TypeKind::Path(PathNode {
                segments: vec![IdentNode {
                    id: DUMMY_NODE_ID,
                    symbol: Symbol::intern("foo"),
                }],
            })),
        );
    }

    #[test]
    fn two() -> TypeKind {
        let source = "foo::bar";

        assert_eq!(
            source,
            Ok(TypeKind::Path(PathNode {
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
            })),
        );
    }

    #[test]
    fn tuple_with_one() -> TypeKind {
        let source = "(foo)";

        assert_eq!(
            source,
            Ok(TypeKind::Tuple(vec![TypeKind::Path(PathNode {
                segments: vec![IdentNode {
                    id: DUMMY_NODE_ID,
                    symbol: Symbol::intern("foo"),
                }],
            })])),
        );
    }

    #[test]
    fn tuple_with_two() -> TypeKind {
        let source = "(foo, bar)";

        assert_eq!(
            source,
            Ok(TypeKind::Tuple(vec![
                TypeKind::Path(PathNode {
                    segments: vec![IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("foo"),
                    }],
                }),
                TypeKind::Path(PathNode {
                    segments: vec![IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("bar"),
                    }],
                }),
            ])),
        );
    }

    #[test]
    fn generic_with_one() -> TypeKind {
        let source = "foo<bar>";

        assert_eq!(
            source,
            Ok(TypeKind::Generic(GenericTypeNode {
                path: PathNode {
                    segments: vec![IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("foo"),
                    }],
                },
                parameters: vec![TypeKind::Path(PathNode {
                    segments: vec![IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("bar"),
                    }],
                })],
            })),
        );
    }

    #[test]
    fn generic_with_two() -> TypeKind {
        let source = "foo<bar, baz>";

        assert_eq!(
            source,
            Ok(TypeKind::Generic(GenericTypeNode {
                path: PathNode {
                    segments: vec![IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("foo"),
                    }],
                },
                parameters: vec![
                    TypeKind::Path(PathNode {
                        segments: vec![IdentNode {
                            id: DUMMY_NODE_ID,
                            symbol: Symbol::intern("bar"),
                        }],
                    }),
                    TypeKind::Path(PathNode {
                        segments: vec![IdentNode {
                            id: DUMMY_NODE_ID,
                            symbol: Symbol::intern("baz"),
                        }],
                    }),
                ],
            })),
        );
    }

    #[test]
    fn tuple_with_generic_with_one() -> TypeKind {
        let source = "(foo<bar>)";

        assert_eq!(
            source,
            Ok(TypeKind::Tuple(vec![TypeKind::Generic(GenericTypeNode {
                path: PathNode {
                    segments: vec![IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("foo"),
                    }],
                },
                parameters: vec![TypeKind::Path(PathNode {
                    segments: vec![IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("bar"),
                    }],
                })],
            })])),
        );
    }

    #[test]
    fn tuple_with_generic_with_two() -> TypeKind {
        let source = "(foo<bar, baz>)";

        assert_eq!(
            source,
            Ok(TypeKind::Tuple(vec![TypeKind::Generic(GenericTypeNode {
                path: PathNode {
                    segments: vec![IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("foo"),
                    }],
                },
                parameters: vec![
                    TypeKind::Path(PathNode {
                        segments: vec![IdentNode {
                            id: DUMMY_NODE_ID,
                            symbol: Symbol::intern("bar"),
                        }],
                    }),
                    TypeKind::Path(PathNode {
                        segments: vec![IdentNode {
                            id: DUMMY_NODE_ID,
                            symbol: Symbol::intern("baz"),
                        }],
                    }),
                ],
            })])),
        );
    }

    #[test]
    fn generic_with_tuple_with_one() -> TypeKind {
        let source = "foo<(bar)>";

        assert_eq!(
            source,
            Ok(TypeKind::Generic(GenericTypeNode {
                path: PathNode {
                    segments: vec![IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("foo"),
                    }],
                },
                parameters: vec![TypeKind::Tuple(vec![TypeKind::Path(PathNode {
                    segments: vec![IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("bar"),
                    }],
                })])],
            })),
        );
    }

    #[test]
    fn generic_with_tuple_with_two() -> TypeKind {
        let source = "foo<(bar, baz)>";

        assert_eq!(
            source,
            Ok(TypeKind::Generic(GenericTypeNode {
                path: PathNode {
                    segments: vec![IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("foo"),
                    }],
                },
                parameters: vec![TypeKind::Tuple(vec![
                    TypeKind::Path(PathNode {
                        segments: vec![IdentNode {
                            id: DUMMY_NODE_ID,
                            symbol: Symbol::intern("bar"),
                        }],
                    }),
                    TypeKind::Path(PathNode {
                        segments: vec![IdentNode {
                            id: DUMMY_NODE_ID,
                            symbol: Symbol::intern("baz"),
                        }],
                    }),
                ])],
            })),
        );
    }
}
