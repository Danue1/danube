use danubec_lex::Lex;
use danubec_syntax::SyntaxKind;

impl crate::Context {
    pub fn literal_expression(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if self.literal(lex) {
            self.start_node_at(checkpoint, SyntaxKind::LiteralExpression);

            self.finish_node();

            true
        } else {
            false
        }
    }
}
