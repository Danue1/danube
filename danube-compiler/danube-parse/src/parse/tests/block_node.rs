use danube_ast::{BlockNode, DUMMY_NODE_ID};

assert_node! {
    #[test]
    fn empty_block() -> BlockNode {
        let source = "{ }";

        assert_eq!(
            source,
            Ok(BlockNode {
                id: DUMMY_NODE_ID,
                statements: vec![],
            }),
        );
    }
}
