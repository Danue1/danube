use danubec_lex::Lex;
use danubec_syntax::{Checkpoint, SyntaxKind};

impl crate::Context {
    pub fn function_call_expression(&mut self, lex: &mut Lex, checkpoint: Checkpoint) -> bool {
        if expect!(self, lex, SyntaxKind::LEFT_PAREN) {
            self.trivia(lex);
            while self.argument(lex) {
                self.trivia(lex);
                if expect!(self, lex, SyntaxKind::COMMA) {
                    self.trivia(lex);
                } else {
                    break;
                }
            }

            self.trivia(lex);
            expect!(self, lex, SyntaxKind::RIGHT_PAREN);

            self.start_node_at(checkpoint, SyntaxKind::FunctionCallExpression);
            self.finish_node();

            true
        } else {
            false
        }
    }
}

#[test]
fn function_call_expression() {
    for source in ["foo()", "foo(1, 2)"] {
        let mut context = crate::Context::new();
        let mut lex = Lex::new(source);
        context.expression(&mut lex);
        let node = context.finish();

        assert_eq!(node.to_string(), source);
    }
}
