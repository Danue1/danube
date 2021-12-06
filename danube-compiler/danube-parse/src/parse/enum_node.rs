use crate::{Error, Parse};
use danube_ast::EnumNode;

impl<'parse> Parse<'parse> {
    pub fn parse_enum_node(&mut self) -> Result<EnumNode, Error> {
        std::todo!();
    }
}
