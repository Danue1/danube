use crate::{Error, Parse};
use danube_ast::{TypeNode, DUMMY_NODE_ID};

impl<'parse> Parse<'parse> {
    pub fn parse_type_node(&mut self) -> Result<TypeNode, Error> {
        let immutability = self.parse_immutability_kind()?;
        let kind = self.parse_type_kind()?;

        Ok(TypeNode {
            id: DUMMY_NODE_ID,
            immutability,
            kind,
        })
    }
}
