use danube_ast::{
    AttributeNode, IdentNode, PackageNode, PathNode, DUMMY_ATTRIBUTE_ID, DUMMY_NODE_ID,
};
use danube_token::Symbol;

assert_node! {
    #[test]
    fn empty() -> PackageNode {
        let source = "";

        assert_eq!(
            source,
            Ok(PackageNode {
                id: DUMMY_NODE_ID,
                attributes: vec![],
                items: vec![],
            }),
        );
    }

    #[test]
    fn attribute() -> PackageNode {
        let source = "#![hello]";

        assert_eq!(
            source,
            Ok(PackageNode {
                id: DUMMY_NODE_ID,
                attributes: vec![AttributeNode {
                    id: DUMMY_ATTRIBUTE_ID,
                    path: PathNode {
                        segments: vec![IdentNode {
                            id: DUMMY_NODE_ID,
                            symbol: Symbol::intern("hello")
                        }]
                    },
                    args: vec![],
                }],
                items: vec![],
            }),
        );
    }

    #[test]
    fn attributes() -> PackageNode {
        let source = "#![hello]\
            #![hello]";

        assert_eq!(
            source,
            Ok(PackageNode {
                id: DUMMY_NODE_ID,
                attributes: vec![
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
                                symbol: Symbol::intern("hello"),
                            }],
                        },
                        args: vec![],
                    },
                ],
                items: vec![],
            }),
        );
    }
}
