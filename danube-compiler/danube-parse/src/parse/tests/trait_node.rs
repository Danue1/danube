use danube_ast::{IdentNode, TraitNode, DUMMY_NODE_ID};
use danube_token::Symbol;

assert_node! {
    #[test]
    fn without_items() -> TraitNode {
        let source = "Foo { }";

        assert_eq!(
            source,
            Ok(TraitNode {
                id: DUMMY_NODE_ID,
                ident: IdentNode {
                    id: DUMMY_NODE_ID,
                    symbol: Symbol::intern("Foo"),
                },
                generics: vec![],
                inheritances: vec![],
                items: vec![],
            }),
        );
    }
}
