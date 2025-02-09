mod function_definition;

use danubec_lex::Lex;
use danubec_syntax::SyntaxKind;

impl crate::Context {
    pub fn definitions(&mut self, lex: &mut Lex) {
        while !lex.is_empty() {
            self.trivia(lex);
            if self.definition(lex) {
                self.trivia(lex);
            } else {
                self.error(lex);
            }
        }
    }

    pub fn definition(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if self.visibility(lex) {
            self.start_node_at(checkpoint, SyntaxKind::Definition);

            self.trivia(lex);
            self.definition_kind(lex);

            self.finish_node();

            true
        } else {
            false
        }
    }

    pub fn definition_kind(&mut self, lex: &mut Lex) -> bool {
        self.function_definition(lex)
    }
}
