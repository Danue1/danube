use crate::{Error, Parse};
use danube_ast::TypeAliasNode;

impl<'parse> Parse<'parse> {
    pub fn parse_type_alias_node(&mut self) -> Result<TypeAliasNode, Error> {
        std::todo!();
    }
}
