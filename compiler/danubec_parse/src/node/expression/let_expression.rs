use danubec_lex::Lex;
use danubec_syntax::SyntaxKind;

impl crate::Context {
    pub fn let_expression(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if expect!(self, lex, SyntaxKind::LET) {
            self.start_node_at(checkpoint, SyntaxKind::LetExpression);

            self.trivia(lex);
            self.identifier(lex);

            self.trivia(lex);
            expect!(self, lex, SyntaxKind::EQUAL);

            self.trivia(lex);
            self.expression(lex);

            self.finish_node();

            true
        } else {
            false
        }
    }
}

#[test]
fn let_expression() {
    for source in ["let a = 42"] {
        let mut context = crate::Context::new();
        let mut lex = Lex::new(source);
        context.start_node(SyntaxKind::Root);
        context.let_expression(&mut lex);
        context.finish_node();
        let node = context.finish();

        assert_eq!(format!("{}", node), source);
    }
}
