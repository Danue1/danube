use danubec_lex::Lex;
use danubec_syntax::SyntaxKind;

impl crate::Context {
    pub fn block_expression(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if expect!(self, lex, SyntaxKind::LEFT_BRACE) {
            self.start_node_at(checkpoint, SyntaxKind::BlockExpression);

            self.trivia(lex);

            while !expect!(self, lex, SyntaxKind::RIGHT_BRACE) {
                self.trivia(lex);

                if self.statement(lex) {
                    self.trivia(lex);
                } else {
                    break;
                }
            }

            self.finish_node();

            true
        } else {
            false
        }
    }
}

#[test]
fn block_expression() {
    for source in ["{ }"] {
        let mut context = crate::Context::new();
        let mut lex = Lex::new(source);
        context.block_expression(&mut lex);
        let node = context.finish();

        assert_eq!(format!("{}", node), source);
    }
}
