use crate::{Context, Error, Parse};
use danube_ast::{PathNode, UseNode};

impl Parse for UseNode {
    type Output = UseNode;

    fn parse(context: &mut Context) -> Result<Self::Output, Error> {
        if let Some(path) = PathNode::parse(context)? {
            if symbol!(context.cursor => Semicolon) {
                Ok(UseNode { path })
            } else {
                Err(Error::Invalid)
            }
        } else {
            Err(Error::Invalid)
        }
    }
}
