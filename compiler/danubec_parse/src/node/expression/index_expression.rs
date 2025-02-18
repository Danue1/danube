use danubec_lex::Lex;
use danubec_syntax::{Checkpoint, SyntaxKind};

impl crate::Context {
    pub fn index_expression(&mut self, lex: &mut Lex, checkpoint: Checkpoint) -> bool {
        if expect!(self, lex, SyntaxKind::LEFT_BRACKET) {
            self.start_node_at(checkpoint, SyntaxKind::IndexExpression);

            self.trivia(lex);
            self.index_element(lex);

            self.trivia(lex);
            expect!(self, lex, SyntaxKind::RIGHT_BRACKET);

            self.finish_node();

            true
        } else {
            false
        }
    }

    fn index_element(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if self.expression(lex) {
            self.start_node_at(checkpoint, SyntaxKind::IndexElement);
            self.finish_node();

            true
        } else {
            false
        }
    }
}

#[test]
fn index_expression() {
    for source in ["foo[1]"] {
        let mut context = crate::Context::new();
        let mut lex = Lex::new(source);
        context.expression(&mut lex);
        let node = context.finish();

        assert_eq!(node.to_string(), source);
    }
}
