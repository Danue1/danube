use super::generic_node::GenericNodeList;
use super::implement_item_node::ImplementItemNodeList;
use crate::{Context, Parse};
use danube_ast::{IdentNode, PathNode, TraitNode, DUMMY_NODE_ID};
use danube_diagnostics::MessageBuilder;

impl Parse for TraitNode {
    type Output = TraitNode;

    fn parse(context: &mut Context) -> Result<Self::Output, ()> {
        let ident = IdentNode::parse(context)?;
        let generics = GenericNodeList::parse(context)?;
        let inheritances = if symbol!(context.cursor => Colon) {
            if let Some(path) = PathNode::parse(context)? {
                let mut inheritances = vec![path];

                while symbol!(context.cursor => Plus) {
                    if let Some(path) = PathNode::parse(context)? {
                        inheritances.push(path);
                    } else {
                        return context
                            .report(MessageBuilder::error("Expected trait path").build());
                    }
                }

                inheritances
            } else {
                return context.report(MessageBuilder::error("Expected trait path").build());
            }
        } else {
            vec![]
        };
        let items = ImplementItemNodeList::parse(context)?;

        Ok(TraitNode {
            id: DUMMY_NODE_ID,
            ident,
            generics,
            inheritances,
            items,
        })
    }
}
