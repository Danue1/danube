use crate::{Context, Parse};
use danube_ast::{BlockNode, StatementNode, DUMMY_NODE_ID};
use danube_diagnostics::MessageBuilder;

impl Parse for BlockNode {
    type Output = BlockNode;

    fn parse(context: &mut Context) -> Result<Self::Output, ()> {
        if !symbol!(context.cursor => LeftBrace) {
            return context.report(MessageBuilder::error("Expected `{`").build());
        }

        let mut statements = vec![];

        while !symbol!(context.cursor => RightBrace) {
            statements.push(StatementNode::parse(context)?);
        }

        Ok(BlockNode {
            id: DUMMY_NODE_ID,
            statements,
        })
    }
}
