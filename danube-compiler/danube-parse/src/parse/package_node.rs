use crate::{Error, Parse};
use danube_ast::PackageNode;

impl<'parse> Parse<'parse> {
    pub fn parse_package_node(&mut self) -> Result<PackageNode, Error> {
        let attributes = self.parse_package_attributes()?;
        let items = self.parse_item_nodes()?;

        Ok(PackageNode { attributes, items })
    }
}
