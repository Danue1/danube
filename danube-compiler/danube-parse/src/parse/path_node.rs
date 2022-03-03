use crate::{Context, Error, Parse};
use danube_ast::{IdentNode, PathNode};

impl Parse for PathNode {
    type Output = Option<PathNode>;

    fn parse(context: &mut Context) -> Result<Self::Output, Error> {
        let mut segments = if let Ok(ident) = IdentNode::parse(context) {
            vec![ident]
        } else {
            return Ok(None);
        };

        while symbol!(context.cursor => ColonColon) {
            segments.push(IdentNode::parse(context)?);
        }

        Ok(Some(PathNode { segments }))
    }
}
