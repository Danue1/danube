mod literal_expression_node;

use crate::State;

impl crate::Context {
    pub fn expression_node(&mut self) -> State {
        one_of!(self.expression_literal_node())
    }
}
