use danubec_lex::Lex;
use danubec_syntax::SyntaxKind;

impl crate::Context {
    pub fn for_expression(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if expect!(self, lex, SyntaxKind::FOR) {
            self.start_node_at(checkpoint, SyntaxKind::ForExpression);

            self.trivia(lex);
            self.expression(lex);

            self.trivia(lex);
            expect!(self, lex, SyntaxKind::IN);

            self.trivia(lex);
            self.expression(lex);

            self.trivia(lex);
            self.block_expression(lex);

            self.finish_node();

            true
        } else {
            false
        }
    }
}

#[test]
fn for_expression() {
    for source in ["for i in 0 {}"] {
        let mut context = crate::Context::new();
        let mut lex = Lex::new(source);
        context.for_expression(&mut lex);
        let node = context.finish();

        assert_eq!(node.to_string(), source);
    }
}
