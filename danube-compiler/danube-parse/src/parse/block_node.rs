use crate::{Error, Parse};
use danube_ast::BlockNode;

impl<'parse> Parse<'parse> {
    pub fn parse_block_node(&mut self) -> Result<BlockNode, Error> {
        if !symbol!(self.cursor => LeftBrace) {
            return Err(Error::Invalid);
        }

        let mut statements = vec![];

        while !symbol!(self.cursor => RightBrace) {
            statements.push(self.parse_statement_node()?);
        }

        Ok(BlockNode { statements })
    }
}
