use danubec_lex::Lex;
use danubec_syntax::SyntaxKind;

impl crate::Context {
    pub fn tuple_pattern(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if expect!(self, lex, SyntaxKind::LEFT_PAREN) {
            self.start_node_at(checkpoint, SyntaxKind::TuplePattern);

            self.trivia(lex);
            while self.pattern(lex) {
                self.trivia(lex);
                if expect!(self, lex, SyntaxKind::COMMA) {
                    self.trivia(lex);
                } else {
                    break;
                }
            }

            self.trivia(lex);
            expect!(self, lex, SyntaxKind::RIGHT_PAREN);

            self.finish_node();

            true
        } else {
            false
        }
    }
}

#[test]
fn tuple_pattern() {
    for source in ["()", "(a)", "(a, b)", "(a, b,)"] {
        let mut context = crate::Context::new();
        let mut lex = Lex::new(source);
        context.tuple_pattern(&mut lex);
        let node = context.finish();

        assert_eq!(format!("{}", node), source);
    }
}
