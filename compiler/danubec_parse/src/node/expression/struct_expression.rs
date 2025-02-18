use danubec_lex::Lex;
use danubec_syntax::{Checkpoint, SyntaxKind};

impl crate::Context {
    pub fn struct_expression(&mut self, lex: &mut Lex, checkpoint: Checkpoint) -> bool {
        if expect!(self, lex, SyntaxKind::LEFT_BRACE) {
            self.start_node_at(checkpoint, SyntaxKind::StructExpression);

            self.trivia(lex);
            while self.struct_expression_field(lex) {
                self.trivia(lex);
                if expect!(self, lex, SyntaxKind::COMMA) {
                    self.trivia(lex);
                } else {
                    break;
                }
            }

            self.trivia(lex);
            expect!(self, lex, SyntaxKind::RIGHT_BRACE);

            self.finish_node();

            true
        } else {
            false
        }
    }

    fn struct_expression_field(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if self.identifier(lex) {
            self.trivia(lex);
            expect!(self, lex, SyntaxKind::COLON);

            self.trivia(lex);
            self.expression(lex);

            self.start_node_at(checkpoint, SyntaxKind::StructExpressionField);
            self.finish_node();

            true
        } else {
            false
        }
    }
}

#[test]
fn struct_expression() {
    for source in [
        "foo {}",
        "foo { bar: 42 }",
        "foo { bar: 42, }",
        "foo { bar: 42, baz: 43 }",
    ] {
        let mut context = crate::Context::new();
        let mut lex = Lex::new(source);
        context.expression(&mut lex);
        let node = context.finish();

        assert_eq!(node.to_string(), source);
    }
}
