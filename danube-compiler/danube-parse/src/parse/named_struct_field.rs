use crate::{Context, Parse};
use danube_ast::{IdentNode, NamedStructField, TypeNode, VisibilityKind};
use danube_diagnostics::MessageBuilder;

impl Parse for NamedStructField {
    type Output = NamedStructField;

    fn parse(context: &mut Context) -> Result<Self::Output, ()> {
        let visibility = VisibilityKind::parse(context)?;
        let ident = IdentNode::parse(context)?;
        let ty = if symbol!(context.cursor => Colon) {
            TypeNode::parse(context)?
        } else {
            return context.report(MessageBuilder::error("Expected `:`").build());
        };

        Ok(NamedStructField {
            visibility,
            ident,
            ty,
        })
    }
}
