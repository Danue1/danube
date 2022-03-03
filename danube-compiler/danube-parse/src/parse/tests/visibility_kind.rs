use danube_ast::{IdentNode, PathNode, VisibilityKind, DUMMY_NODE_ID};
use danube_token::Symbol;

assert_node! {
    #[test]
    fn current() -> VisibilityKind {
        let source = "";

        assert_eq!(
            source,
            Ok(VisibilityKind::Current),
        );
    }

    #[test]
    fn public() -> VisibilityKind {
        let source = "pub";

        assert_eq!(
            source,
            Ok(VisibilityKind::Public),
        );
    }

    #[test]
    fn restricted() -> VisibilityKind {
        let source = "pub(foo)";

        assert_eq!(
            source,
            Ok(VisibilityKind::Restricted(PathNode {
                segments: vec![IdentNode {
                    id: DUMMY_NODE_ID,
                    symbol: Symbol::intern("foo"),
                }],
            })),
        );
    }
}
