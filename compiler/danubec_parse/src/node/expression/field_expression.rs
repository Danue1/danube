use danubec_lex::Lex;
use danubec_syntax::{Checkpoint, SyntaxKind};

impl crate::Context {
    pub fn field_expression(&mut self, lex: &mut Lex, checkpoint: Checkpoint) -> bool {
        let path_checkpoint = self.checkpoint();
        if self.identifier(lex) {
            self.trivia(lex);
            if !self.method_call_expression(lex, checkpoint, path_checkpoint) {
                self.start_node_at(checkpoint, SyntaxKind::FieldExpression);
                self.finish_node();
            }

            true
        } else {
            false
        }
    }
}

#[test]
fn field_expression() {
    for source in ["foo.bar"] {
        let mut context = crate::Context::new();
        let mut lex = Lex::new(source);
        context.expression(&mut lex);
        let node = context.finish();

        dbg!(&node);
        assert_eq!(node.to_string(), source);
    }
}
