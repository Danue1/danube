use crate::{Error, Parse};
use danube_ast::ConstantNode;

impl<'parse> Parse<'parse> {
    pub fn parse_constant_node(&mut self) -> Result<ConstantNode, Error> {
        let path = self.parse_path_node()?;

        std::todo!();
    }
}
