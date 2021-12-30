use crate::{Error, Parse};
use danube_ast::IdentNode;

impl<'parse> Parse<'parse> {
    pub fn parse_ident_node(&mut self) -> Result<IdentNode, Error> {
        match identifier!(self.cursor) {
            Some(symbol) => {
                self.cursor.next();

                Ok(IdentNode { symbol })
            }
            None => Err(Error::Invalid),
        }
    }
}
