use danubec_lex::Lex;
use danubec_syntax::SyntaxKind;

impl crate::Context {
    pub fn definition_statement(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if self.definition(lex) {
            self.start_node_at(checkpoint, SyntaxKind::DefinitionStatement);
            self.finish_node();

            true
        } else {
            false
        }
    }
}
