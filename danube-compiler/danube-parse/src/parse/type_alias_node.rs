use crate::{Error, Parse};
use danube_ast::TypeAliasNode;

impl<'parse> Parse<'parse> {
    pub fn parse_type_alias_node(&mut self) -> Result<TypeAliasNode, Error> {
        let ident = self.parse_ident_node()?;

        if symbol!(self.cursor => Eq) {
            let ty = self.parse_type_node()?;

            if symbol!(self.cursor => Semicolon) {
                return Ok(TypeAliasNode { ident, ty });
            }
        }

        Err(Error::Invalid)
    }
}
