use crate::{Error, Parse};
use danube_ast::PathNode;

impl<'parse> Parse<'parse> {
    pub fn parse_path_node(&mut self) -> Result<PathNode, Error> {
        let mut idents = vec![];
        loop {
            idents.push(self.parse_ident_node()?);

            if !symbol!(self.cursor => ColonColon) {
                break;
            }
        }

        Ok(PathNode { idents })
    }
}
