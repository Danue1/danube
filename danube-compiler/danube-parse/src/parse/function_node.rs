use crate::{Error, Parse};
use danube_ast::FunctionNode;

impl<'parse> Parse<'parse> {
    pub fn parse_function_node(&mut self) -> Result<FunctionNode, Error> {
        std::todo!();
    }
}
