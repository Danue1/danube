use crate::{Context, Error, Parse};
use danube_ast::{IdentNode, TypeAliasNode, TypeNode};

impl Parse for TypeAliasNode {
    type Output = TypeAliasNode;

    fn parse(context: &mut Context) -> Result<Self::Output, Error> {
        let ident = IdentNode::parse(context)?;

        if symbol!(context.cursor => Eq) {
            let ty = TypeNode::parse(context)?;

            if symbol!(context.cursor => Semicolon) {
                Ok(TypeAliasNode {
                    ident,
                    ty: Some(ty),
                })
            } else {
                Err(Error::Invalid)
            }
        } else {
            Ok(TypeAliasNode { ident, ty: None })
        }
    }
}
