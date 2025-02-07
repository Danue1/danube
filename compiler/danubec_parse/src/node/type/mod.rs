mod path_type;

use danubec_lex::Lex;

impl crate::Context {
    pub fn ty(&mut self, lex: &mut Lex) -> bool {
        self.path_type(lex)
    }
}
