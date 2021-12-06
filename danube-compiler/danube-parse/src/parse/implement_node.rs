use crate::{Error, Parse};
use danube_ast::ImplementNode;

impl<'parse> Parse<'parse> {
    pub fn parse_implement_node(&mut self) -> Result<ImplementNode, Error> {
        std::todo!();
    }
}
