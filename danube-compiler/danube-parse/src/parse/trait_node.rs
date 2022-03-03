use crate::{Context, Error, Parse, ParseList};
use danube_ast::{GenericNode, IdentNode, ImplementItemNode, PathNode, TraitNode};

impl Parse for TraitNode {
    type Output = TraitNode;

    fn parse(context: &mut Context) -> Result<Self::Output, Error> {
        let ident = IdentNode::parse(context)?;
        let generics = GenericNode::parse_list(context)?;
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
        let items = ImplementItemNode::parse_list(context)?;

        Ok(TraitNode {
            ident,
            generics,
            inheritances,
            items,
        })
    }
}
