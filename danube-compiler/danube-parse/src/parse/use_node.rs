use crate::{Error, Parse};
use danube_ast::UseNode;

impl<'parse> Parse<'parse> {
    pub fn parse_use_node(&mut self) -> Result<UseNode, Error> {
        let path = self.parse_path_node()?;
        if symbol!(self.cursor => Semicolon) {
            Ok(UseNode { path })
        } else {
            Err(Error::Invalid)
        }
    }
}
