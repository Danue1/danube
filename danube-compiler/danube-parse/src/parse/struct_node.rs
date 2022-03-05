use super::generic_node::GenericNodeList;
use crate::{Context, Parse};
use danube_ast::{IdentNode, StructFieldKind, StructNode, DUMMY_NODE_ID};

impl Parse for StructNode {
    type Output = StructNode;

    fn parse(context: &mut Context) -> Result<Self::Output, ()> {
        let ident = IdentNode::parse(context)?;
        let generics = GenericNodeList::parse(context)?;
        let fields = StructFieldKind::parse(context)?;

        Ok(StructNode {
            id: DUMMY_NODE_ID,
            ident,
            generics,
            fields,
        })
    }
}
