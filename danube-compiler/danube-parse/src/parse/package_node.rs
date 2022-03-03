use super::attribute_node::PackageAttributeNode;
use crate::{Context, Error, Parse, ParseList};
use danube_ast::{ItemNode, PackageNode, DUMMY_NODE_ID};

impl Parse for PackageNode {
    type Output = PackageNode;

    fn parse(context: &mut Context) -> Result<Self::Output, Error> {
        Ok(PackageNode {
            id: DUMMY_NODE_ID,
            attributes: PackageAttributeNode::parse_list(context)?,
            items: ItemNode::parse_list(context)?,
        })
    }
}
