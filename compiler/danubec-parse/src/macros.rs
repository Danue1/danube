#[macro_export]
macro_rules! assert_node {
    ($tokens:expr, $text:expr,) => {
        let mut tokens = $tokens;
        tokens.reverse();
        let mut context = crate::Context::new(tokens);
        context.ast();
        let node = danubec_syntax_node::SyntaxNode::new_root(context.builder.finish());
        dbg!(format!("{}", &node));
        dbg!(&node);
        assert_eq!(format!("{:#?}", node), $text);
    };
}
