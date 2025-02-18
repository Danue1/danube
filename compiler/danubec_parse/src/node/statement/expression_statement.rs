use danubec_lex::Lex;
use danubec_syntax::SyntaxKind;

impl crate::Context {
    pub fn expression_statement(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if self.expression_top(lex) {
            self.start_node_at(checkpoint, SyntaxKind::ExpressionStatement);

            self.trivia(lex);
            expect!(self, lex, SyntaxKind::SEMICOLON);

            self.finish_node();

            true
        } else {
            false
        }
    }
}
