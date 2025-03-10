use danubec_lex::Lex;
use danubec_syntax::SyntaxKind;

impl crate::Context {
    pub fn array_expression(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if expect!(self, lex, SyntaxKind::LEFT_BRACKET) {
            self.start_node_at(checkpoint, SyntaxKind::ArrayExpression);
            self.trivia(lex);
            while !expect!(self, lex, SyntaxKind::RIGHT_BRACKET) {
                self.trivia(lex);
                if self.expression(lex) {
                    self.trivia(lex);
                    if !expect!(self, lex, SyntaxKind::COMMA) {
                        self.trivia(lex);
                        expect!(self, lex, SyntaxKind::RIGHT_BRACKET);
                        break;
                    }
                } else {
                    self.error(lex);
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
fn array_expression() {
    for source in ["[]", "[false]", "[true, false]", "[true, false,]"] {
        let mut context = crate::Context::new();
        let mut lex = Lex::new(source);
        context.array_expression(&mut lex);
        let node = context.finish();

        assert_eq!(format!("{}", node), source);
    }
}
