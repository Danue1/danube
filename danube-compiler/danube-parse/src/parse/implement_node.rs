use super::generic_node::GenericNodeList;
use super::implement_item_node::ImplementItemNodeList;
use crate::{Context, Error, Parse};
use danube_ast::{ImplementNode, PathNode};

impl Parse for ImplementNode {
    type Output = ImplementNode;

    fn parse(context: &mut Context) -> Result<Self::Output, Error> {
        let generics = GenericNodeList::parse(context)?;
        let trait_ident = if let Some(path) = PathNode::parse(context)? {
            path
        } else {
            return Err(Error::Invalid);
        };
        let (trait_ident, target) = if identifier!(context.cursor => For) {
            if let Some(path) = PathNode::parse(context)? {
                (Some(trait_ident), path)
            } else {
                return Err(Error::Invalid);
            }
        } else {
            (None, trait_ident)
        };
        let target_generics = GenericNodeList::parse(context)?;
        let items = ImplementItemNodeList::parse(context)?;

        Ok(ImplementNode {
            generics,
            trait_ident,
            target,
            target_generics,
            items,
        })
    }
}
