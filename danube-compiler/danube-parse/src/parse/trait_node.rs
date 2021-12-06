use crate::{Error, Parse};
use danube_ast::TraitNode;

impl<'parse> Parse<'parse> {
    pub fn parse_trait_node(&mut self) -> Result<TraitNode, Error> {
        std::todo!();
    }
}
