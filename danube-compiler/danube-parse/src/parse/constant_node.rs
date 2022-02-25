use crate::{Error, Parse};
use danube_ast::ConstantNode;

impl<'parse> Parse<'parse> {
    pub fn parse_constant_node(&mut self) -> Result<ConstantNode, Error> {
        let pattern = self.parse_pattern_node()?;
        let ty = if symbol!(self.cursor => Colon) {
            self.parse_type_node()?
        } else {
            return Err(Error::Invalid);
        };
        let expression = if symbol!(self.cursor => Eq) {
            Some(self.parse_expression_node()?)
        } else {
            None
        };

        if symbol!(self.cursor => Semicolon) {
            Ok(ConstantNode {
                pattern,
                ty,
                expression,
            })
        } else {
            Err(Error::Invalid)
        }
    }
}
