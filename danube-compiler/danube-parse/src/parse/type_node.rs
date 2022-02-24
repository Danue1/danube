use crate::{Error, Parse};
use danube_ast::TypeNode;

impl<'parse> Parse<'parse> {
  pub fn parse_type_node(&mut self) -> Result<TypeNode, Error> {
    let immutablity = self.parse_immutablity_kind()?;
    let kind = self.parse_type_kind()?;

    Ok(TypeNode { immutablity, kind })
  }
}
