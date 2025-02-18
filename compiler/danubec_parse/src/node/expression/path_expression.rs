use danubec_lex::Lex;
use danubec_syntax::SyntaxKind;

impl crate::Context {
    pub fn path_expression(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if self.path(lex) {
            if !self.struct_expression(lex, checkpoint) {
                self.start_node_at(checkpoint, SyntaxKind::PathExpression);
                self.finish_node();
            }

            true
        } else {
            false
        }
    }
}
