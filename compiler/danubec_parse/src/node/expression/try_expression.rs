use danubec_lex::Lex;
use danubec_syntax::{Checkpoint, SyntaxKind};

impl crate::Context {
    pub fn try_expression(&mut self, lex: &mut Lex, checkpoint: Checkpoint) -> bool {
        if expect!(self, lex, SyntaxKind::QUESTION) {
            self.start_node_at(checkpoint, SyntaxKind::TryExpression);
            self.finish_node();

            true
        } else {
            false
        }
    }
}

#[test]
fn try_expression() {
    for source in ["foo?", "foo??"] {
        let mut context = crate::Context::new();
        let mut lex = Lex::new(source);
        context.expression(&mut lex);
        let node = context.finish();

        assert_eq!(node.to_string(), source);
    }
}
