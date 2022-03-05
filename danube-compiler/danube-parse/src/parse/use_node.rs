use crate::{Context, Parse};
use danube_ast::{PathNode, UseNode, DUMMY_NODE_ID};
use danube_diagnostics::MessageBuilder;

impl Parse for UseNode {
    type Output = UseNode;

    fn parse(context: &mut Context) -> Result<Self::Output, ()> {
        if let Some(path) = PathNode::parse(context)? {
            if symbol!(context.cursor => Semicolon) {
                Ok(UseNode {
                    id: DUMMY_NODE_ID,
                    path,
                })
            } else {
                context.report(MessageBuilder::error("Expected `;`").build())
            }
        } else {
            context.report(MessageBuilder::error("Expected type path").build())
        }
    }
}
