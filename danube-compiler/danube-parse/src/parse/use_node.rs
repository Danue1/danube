use crate::{Error, Parse};
use danube_ast::UseNode;

impl<'parse> Parse<'parse> {
    pub fn parse_use_node(&mut self) -> Result<UseNode, Error> {
        let path = if let Some(path) = self.parse_path_node()? {
            path
        } else {
            return Err(Error::Invalid);
        };
        if symbol!(self.cursor => Semicolon) {
            Ok(UseNode { path })
        } else {
            Err(Error::Invalid)
        }
    }
}
