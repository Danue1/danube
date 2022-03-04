use super::generic_node::GenericNodeList;
use crate::{Context, Parse};
use danube_ast::{IdentNode, StructFieldKind, StructNode};

impl Parse for StructNode {
    type Output = StructNode;

    fn parse(context: &mut Context) -> Result<Self::Output, ()> {
        let ident = IdentNode::parse(context)?;
        let generics = GenericNodeList::parse(context)?;
        let fields = StructFieldKind::parse(context)?;

        Ok(StructNode {
            ident,
            generics,
            fields,
        })
    }
}
