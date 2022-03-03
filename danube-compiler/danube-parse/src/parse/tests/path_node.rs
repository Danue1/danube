use danube_ast::{IdentNode, PathNode, DUMMY_NODE_ID};
use danube_token::Symbol;

assert_node! {
    #[test]
    fn one() -> PathNode {
        let source = "one";

        assert_eq!(
            source,
            Ok(Some(PathNode {
                segments: vec![IdentNode {
                    id: DUMMY_NODE_ID,
                    symbol: Symbol::intern("one"),
                }],
            })),
        );
    }

    #[test]
    fn onw_two() -> PathNode {
        let source = "one::two";

        assert_eq!(
            source,
            Ok(Some(PathNode {
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
            })),
        );
    }
}
