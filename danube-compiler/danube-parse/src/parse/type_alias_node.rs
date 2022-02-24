use crate::{Error, Parse};
use danube_ast::TypeAliasNode;

impl<'parse> Parse<'parse> {
    pub fn parse_type_alias_node(&mut self) -> Result<TypeAliasNode, Error> {
        let ident = self.parse_ident_node()?;

        if symbol!(self.cursor => Eq) {
            let ty = self.parse_type_node()?;

            if symbol!(self.cursor => Semicolon) {
                Ok(TypeAliasNode {
                    ident,
                    ty: Some(ty),
                })
            } else {
                Err(Error::Invalid)
            }
        } else {
            Ok(TypeAliasNode { ident, ty: None })
        }
    }
}
