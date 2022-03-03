use crate::{Context, Parse};
use danube_ast::{IdentNode, TypeAliasNode, TypeNode};
use danube_diagnostics::MessageBuilder;

impl Parse for TypeAliasNode {
    type Output = TypeAliasNode;

    fn parse(context: &mut Context) -> Result<Self::Output, ()> {
        let ident = IdentNode::parse(context)?;

        if symbol!(context.cursor => Eq) {
            let ty = TypeNode::parse(context)?;

            if symbol!(context.cursor => Semicolon) {
                Ok(TypeAliasNode {
                    ident,
                    ty: Some(ty),
                })
            } else {
                context.report(MessageBuilder::error("Expected `;`").build())
            }
        } else {
            Ok(TypeAliasNode { ident, ty: None })
        }
    }
}
