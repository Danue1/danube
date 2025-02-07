mod function_definition;

use danubec_lex::Lex;

impl crate::Context {
    pub fn definitions(&mut self, lex: &mut Lex) {
        while !lex.is_empty() {
            if self.definition(lex) {
                self.trivia(lex);
            } else {
                self.error(lex);
            }
        }
    }

    pub fn definition(&mut self, lex: &mut Lex) -> bool {
        self.function_definition(lex)
    }
}
