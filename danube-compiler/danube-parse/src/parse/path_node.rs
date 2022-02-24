use crate::{Error, Parse};
use danube_ast::PathNode;

impl<'parse> Parse<'parse> {
    pub fn parse_path_node(&mut self) -> Result<Option<PathNode>, Error> {
        let mut segments = if let Ok(ident) = self.parse_ident_node() {
            vec![ident]
        } else {
            return Ok(None);
        };

        while symbol!(self.cursor => ColonColon) {
            segments.push(self.parse_ident_node()?);
        }

        Ok(Some(PathNode { segments }))
    }
}
