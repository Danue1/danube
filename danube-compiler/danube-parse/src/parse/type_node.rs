use crate::{Error, Parse};
use danube_ast::TypeNode;

impl<'parse> Parse<'parse> {
  pub fn parse_type_node(&mut self) -> Result<TypeNode, Error> {
    std::todo!();
  }
}
