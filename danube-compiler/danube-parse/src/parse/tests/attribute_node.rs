use crate::parse::attribute_node::{ItemAttributeNodeList, PackageAttributeNodeList};
use danube_ast::{
    AttributeArgumentNode, AttributeNode, ExpressionKind, ExpressionNode, IdentNode, PathNode,
    DUMMY_ATTRIBUTE_ID, DUMMY_NODE_ID,
};
use danube_token::Symbol;

assert_node! {
    #[test]
    fn package_attribute() -> PackageAttributeNodeList {
        let source = "#![hello]";

        assert_eq!(
            source,
            Ok(vec![AttributeNode {
                id: DUMMY_ATTRIBUTE_ID,
                path: PathNode {
                    segments: vec![IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("hello"),
                    }],
                },
                args: vec![],
            }]),
        );
    }

    #[test]
    fn package_attributes() -> PackageAttributeNodeList {
        let source = "#![hello] #![world]";

        assert_eq!(
            source,
            Ok(vec![
                AttributeNode {
                    id: DUMMY_ATTRIBUTE_ID,
                    path: PathNode {
                        segments: vec![IdentNode {
                            id: DUMMY_NODE_ID,
                            symbol: Symbol::intern("hello"),
                        }],
                    },
                    args: vec![],
                },
                AttributeNode {
                    id: DUMMY_ATTRIBUTE_ID,
                    path: PathNode {
                        segments: vec![IdentNode {
                            id: DUMMY_NODE_ID,
                            symbol: Symbol::intern("world"),
                        }]
                    },
                    args: vec![],
                },
            ]),
        );
    }

    #[test]
    fn item_attribute() -> ItemAttributeNodeList {
        let source = "#[hello]";

        assert_eq!(
            source,
            Ok(vec![AttributeNode {
                id: DUMMY_ATTRIBUTE_ID,
                path: PathNode {
                    segments: vec![IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("hello"),
                    }],
                },
                args: vec![],
            }]),
        );
    }

    #[test]
    fn item_attributes() -> ItemAttributeNodeList {
        let source = "#[hello] #[world]";

        assert_eq!(
            source,
            Ok(vec![
                AttributeNode {
                    id: DUMMY_ATTRIBUTE_ID,
                    path: PathNode {
                        segments: vec![IdentNode {
                            id: DUMMY_NODE_ID,
                            symbol: Symbol::intern("hello"),
                        }],
                    },
                    args: vec![],
                },
                AttributeNode {
                    id: DUMMY_ATTRIBUTE_ID,
                    path: PathNode {
                        segments: vec![IdentNode {
                            id: DUMMY_NODE_ID,
                            symbol: Symbol::intern("world"),
                        }],
                    },
                    args: vec![],
                },
            ]),
        );
    }

    #[test]
    fn item_attribute_with_argument() -> ItemAttributeNodeList {
        let source = "#[hello(foo)]";

        assert_eq!(
            source,
            Ok(vec![AttributeNode {
                id: DUMMY_ATTRIBUTE_ID,
                path: PathNode {
                    segments: vec![IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("hello"),
                    }],
                },
                args: vec![AttributeArgumentNode{
                    ident: IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("foo"),
                    },
                    value: None,
                }],
            }]),
        );
    }

    #[test]
    fn item_attribute_with_argument_and_expression() -> ItemAttributeNodeList {
        let source = "#[hello(foo = bar)]";

        assert_eq!(
            source,
            Ok(vec![AttributeNode {
                id: DUMMY_ATTRIBUTE_ID,
                path: PathNode {
                    segments: vec![IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("hello"),
                    }],
                },
                args: vec![AttributeArgumentNode {
                    ident: IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("foo"),
                    },
                    value: Some(ExpressionNode {
                        id: DUMMY_NODE_ID,
                        kind: ExpressionKind::Path(PathNode {
                            segments: vec![IdentNode {
                                id: DUMMY_NODE_ID,
                                symbol: Symbol::intern("bar"),
                            }],
                        }),
                    }),
                }],
            }]),
        );
    }
}
