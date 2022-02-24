use crate::{Error, Parse};
use danube_ast::TraitNode;

impl<'parse> Parse<'parse> {
    pub fn parse_trait_node(&mut self) -> Result<TraitNode, Error> {
        let ident = self.parse_ident_node()?;
        let generics = self.parse_generic_nodes()?;
        let inheritances = if symbol!(self.cursor => Colon) {
            if let Some(path) = self.parse_path_node()? {
                let mut inheritances = vec![path];

                while symbol!(self.cursor => Plus) {
                    if let Some(path) = self.parse_path_node()? {
                        inheritances.push(path);
                    } else {
                        return Err(Error::Invalid);
                    }
                }

                inheritances
            } else {
                return Err(Error::Invalid);
            }
        } else {
            vec![]
        };
        let items = self.parse_implement_item_nodes()?;

        Ok(TraitNode {
            ident,
            generics,
            inheritances,
            items,
        })
    }
}
