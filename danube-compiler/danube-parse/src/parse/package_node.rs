use super::attribute_node::PackageAttributeNodeList;
use super::item_node::ItemNodeList;
use crate::{Context, Error, Parse};
use danube_ast::{PackageNode, DUMMY_NODE_ID};

impl Parse for PackageNode {
    type Output = PackageNode;

    fn parse(context: &mut Context) -> Result<Self::Output, Error> {
        Ok(PackageNode {
            id: DUMMY_NODE_ID,
            attributes: PackageAttributeNodeList::parse(context)?,
            items: ItemNodeList::parse(context)?,
        })
    }
}
