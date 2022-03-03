use danube_ast::{IdentNode, DUMMY_NODE_ID};
use danube_token::Symbol;

assert_node! {
    #[test]
    fn ident_node() -> IdentNode {
        let source = "hello";

        assert_eq!(
            source,
            Ok(IdentNode {
                id: DUMMY_NODE_ID,
                symbol: Symbol::intern("hello"),
            }),
        );
    }
}
