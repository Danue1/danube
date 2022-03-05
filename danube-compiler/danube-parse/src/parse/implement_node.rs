use super::generic_node::GenericNodeList;
use super::implement_item_node::ImplementItemNodeList;
use crate::{Context, Parse};
use danube_ast::{ImplementNode, PathNode, DUMMY_NODE_ID};
use danube_diagnostics::MessageBuilder;

impl Parse for ImplementNode {
    type Output = ImplementNode;

    fn parse(context: &mut Context) -> Result<Self::Output, ()> {
        let generics = GenericNodeList::parse(context)?;
        let trait_ident = if let Some(path) = PathNode::parse(context)? {
            path
        } else {
            return context.report(MessageBuilder::error("Expected trait path").build());
        };
        let (trait_ident, target) = if identifier!(context.cursor => For) {
            if let Some(path) = PathNode::parse(context)? {
                (Some(trait_ident), path)
            } else {
                return context.report(MessageBuilder::error("Expected target type").build());
            }
        } else {
            (None, trait_ident)
        };
        let target_generics = GenericNodeList::parse(context)?;
        let items = ImplementItemNodeList::parse(context)?;

        Ok(ImplementNode {
            id: DUMMY_NODE_ID,
            generics,
            trait_ident,
            target,
            target_generics,
            items,
        })
    }
}
