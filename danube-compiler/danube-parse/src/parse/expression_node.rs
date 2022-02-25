use crate::{Error, Parse};
use danube_ast::{ExpressionNode, DUMMY_NODE_ID};

impl<'parse> Parse<'parse> {
    pub fn parse_expression_node(&mut self) -> Result<ExpressionNode, Error> {
        Ok(ExpressionNode {
            id: DUMMY_NODE_ID,
            kind: self.parse_expression_kind()?,
        })
    }

    pub(crate) fn parse_prefix_expression_node(&mut self) -> Result<ExpressionNode, Error> {
        Ok(ExpressionNode {
            id: DUMMY_NODE_ID,
            kind: self.parse_prefix_expression_kind()?,
        })
    }
}
