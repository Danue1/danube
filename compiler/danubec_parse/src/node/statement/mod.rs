mod definition_statement;
mod expression_statement;
mod let_statement;
mod semicolon_statement;

use danubec_lex::Lex;

impl crate::Context {
    pub fn statement(&mut self, lex: &mut Lex) -> bool {
        self.definition_statement(lex)
            || self.expression_statement(lex)
            || self.let_statement(lex)
            || self.semicolon_statement(lex)
    }
}
