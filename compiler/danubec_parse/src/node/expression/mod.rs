mod assignment_expression;
mod block_expression;
mod let_expression;
mod literal_expression;

use danubec_lex::Lex;

impl crate::Context {
    pub fn expression(&mut self, lex: &mut Lex) -> bool {
        self.literal_expression(lex)
            || self.block_expression(lex)
            || self.let_expression(lex)
            || self.assignment_expression(lex)
    }
}
