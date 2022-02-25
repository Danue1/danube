use crate::{Error, Parse};
use danube_ast::{IdentNode, DUMMY_NODE_ID};

impl<'parse> Parse<'parse> {
    pub fn parse_ident_node(&mut self) -> Result<IdentNode, Error> {
        match identifier!(self.cursor) {
            Some(symbol) => {
                self.cursor.next();

                Ok(IdentNode {
                    id: DUMMY_NODE_ID,
                    symbol,
                })
            }
            None => Err(Error::Invalid),
        }
    }
}
