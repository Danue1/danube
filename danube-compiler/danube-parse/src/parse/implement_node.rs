use crate::{Context, Error, Parse, ParseList};
use danube_ast::{GenericNode, ImplementItemNode, ImplementNode, PathNode};

impl Parse for ImplementNode {
    type Output = ImplementNode;

    fn parse(context: &mut Context) -> Result<Self::Output, Error> {
        let generics = GenericNode::parse_list(context)?;
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
        let target_generics = GenericNode::parse_list(context)?;
        let items = ImplementItemNode::parse_list(context)?;

        Ok(ImplementNode {
            generics,
            trait_ident,
            target,
            target_generics,
            items,
        })
    }
}
