use danubec_lex::Lex;
use danubec_syntax::{Checkpoint, SyntaxKind};

impl crate::Context {
    pub fn method_call_expression(
        &mut self,
        lex: &mut Lex,
        checkpoint: Checkpoint,
        path_checkpoint: Checkpoint,
    ) -> bool {
        let mut matched = false;
        if self.type_argument(lex) {
            matched = true;
            self.trivia(lex);
        }

        if lex.matches(|kind, _| kind == SyntaxKind::LEFT_PAREN) {
            self.start_node_at(path_checkpoint, SyntaxKind::PathSegment);
            self.finish_node();
        }

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

            matched = true;
        }

        if matched {
            self.start_node_at(checkpoint, SyntaxKind::MethodCallExpression);
            self.finish_node();
        }

        matched
    }

    pub fn argument(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if self.expression(lex) {
            self.start_node_at(checkpoint, SyntaxKind::Argument);
            self.finish_node();

            true
        } else {
            false
        }
    }
}

#[test]
fn method_call_expression() {
    for source in [
        "foo.bar()",
        "foo.bar<T>()",
        "foo.bar(1, 2)",
        "foo().bar(1, 2)",
    ] {
        let mut context = crate::Context::new();
        let mut lex = Lex::new(source);
        context.expression(&mut lex);
        let node = context.finish();

        dbg!(&node);
        assert_eq!(node.to_string(), source);
    }
}
