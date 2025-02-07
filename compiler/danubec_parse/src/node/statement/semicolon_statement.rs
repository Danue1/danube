use danubec_lex::Lex;
use danubec_syntax::SyntaxKind;

impl crate::Context {
    pub fn semicolon_statement(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if expect!(self, lex, SyntaxKind::SEMICOLON) {
            self.start_node_at(checkpoint, SyntaxKind::SemicolonStatement);
            self.finish_node();

            true
        } else {
            false
        }
    }
}
