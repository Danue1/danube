use crate::{Context, Parse};
use danube_ast::{IdentNode, DUMMY_NODE_ID};
use danube_diagnostics::MessageBuilder;

impl Parse for IdentNode {
    type Output = IdentNode;

    fn parse(context: &mut Context) -> Result<Self::Output, ()> {
        match identifier!(context.cursor) {
            Some(symbol) => {
                context.cursor.next();

                Ok(IdentNode {
                    id: DUMMY_NODE_ID,
                    symbol,
                })
            }
            None => context.report(MessageBuilder::error("Expected identifier").build()),
        }
    }
}
