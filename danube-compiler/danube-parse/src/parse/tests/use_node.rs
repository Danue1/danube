use danube_ast::{IdentNode, PathNode, UseNode, DUMMY_NODE_ID};
use danube_token::Symbol;

assert_node! {
    #[test]
    fn one() -> UseNode {
        let source = "one;";

        assert_eq!(
            source,
            Ok(UseNode {
                path: PathNode {
                    segments: vec![IdentNode {
                        id: DUMMY_NODE_ID,
                        symbol: Symbol::intern("one"),
                    }],
                },
            }),
        );
    }

    #[test]
    fn two() -> UseNode {
        let source = "one::two;";

        assert_eq!(
            source,
            Ok(UseNode {
                path: PathNode {
                    segments: vec![
                        IdentNode {
                            id: DUMMY_NODE_ID,
                            symbol: Symbol::intern("one"),
                        },
                        IdentNode {
                            id: DUMMY_NODE_ID,
                            symbol: Symbol::intern("two"),
                        },
                    ],
                },
            }),
        );
    }
}
