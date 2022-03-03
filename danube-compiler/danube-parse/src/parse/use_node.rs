use crate::{Context, Parse};
use danube_ast::{PathNode, UseNode};
use danube_diagnostics::MessageBuilder;

impl Parse for UseNode {
    type Output = UseNode;

    fn parse(context: &mut Context) -> Result<Self::Output, ()> {
        if let Some(path) = PathNode::parse(context)? {
            if symbol!(context.cursor => Semicolon) {
                Ok(UseNode { path })
            } else {
                context.report(MessageBuilder::error("Expected `;`").build())
            }
        } else {
            context.report(MessageBuilder::error("Expected type path").build())
        }
    }
}
