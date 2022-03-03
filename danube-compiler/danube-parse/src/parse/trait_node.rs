use super::generic_node::GenericNodeList;
use super::implement_item_node::ImplementItemNodeList;
use crate::{Context, Error, Parse};
use danube_ast::{IdentNode, PathNode, TraitNode};

impl Parse for TraitNode {
    type Output = TraitNode;

    fn parse(context: &mut Context) -> Result<Self::Output, Error> {
        let ident = IdentNode::parse(context)?;
        let generics = GenericNodeList::parse(context)?;
        let inheritances = if symbol!(context.cursor => Colon) {
            if let Some(path) = PathNode::parse(context)? {
                let mut inheritances = vec![path];

                while symbol!(context.cursor => Plus) {
                    if let Some(path) = PathNode::parse(context)? {
                        inheritances.push(path);
                    } else {
                        return Err(Error::Invalid);
                    }
                }

                inheritances
            } else {
                return Err(Error::Invalid);
            }
        } else {
            vec![]
        };
        let items = ImplementItemNodeList::parse(context)?;

        Ok(TraitNode {
            ident,
            generics,
            inheritances,
            items,
        })
    }
}
