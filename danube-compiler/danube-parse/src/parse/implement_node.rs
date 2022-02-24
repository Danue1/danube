use crate::{Error, Parse};
use danube_ast::ImplementNode;

impl<'parse> Parse<'parse> {
    pub fn parse_implement_node(&mut self) -> Result<ImplementNode, Error> {
        let generics = self.parse_generic_nodes()?;
        let trait_ident = if let Some(path) = self.parse_path_node()? {
            path
        } else {
            return Err(Error::Invalid);
        };
        let (trait_ident, target) = if identifier!(self.cursor => For) {
            let path = if let Some(path) = self.parse_path_node()? {
                path
            } else {
                return Err(Error::Invalid);
            };
            (Some(trait_ident), path)
        } else {
            (None, trait_ident)
        };
        let target_generics = self.parse_generic_nodes()?;

        if !symbol!(self.cursor => LeftBrace) {
            return Err(Error::Invalid);
        }

        let items = self.parse_implement_item_nodes()?;

        Ok(ImplementNode {
            generics,
            trait_ident,
            target,
            target_generics,
            items,
        })
    }
}
