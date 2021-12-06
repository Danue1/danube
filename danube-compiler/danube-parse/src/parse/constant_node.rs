use crate::{Error, Parse};
use danube_ast::ConstantNode;

impl<'parse> Parse<'parse> {
    pub fn parse_constant_node(&mut self) -> Result<ConstantNode, Error> {
        std::todo!();
    }
}
