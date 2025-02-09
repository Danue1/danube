mod array_literal;
mod boolean_literal;
mod char_literal;
mod numeric_literal;
mod string_literal;

use danubec_lex::Lex;

impl crate::Context {
    pub fn literal(&mut self, lex: &mut Lex) -> bool {
        self.array_literal(lex)
            || self.boolean_literal(lex)
            || self.char_literal(lex)
            || self.numeric_literal(lex)
            || self.string_literal(lex)
    }
}
