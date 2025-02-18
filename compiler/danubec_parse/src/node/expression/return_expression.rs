use danubec_lex::Lex;
use danubec_syntax::SyntaxKind;

impl crate::Context {
    pub fn return_expression(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if expect!(self, lex, SyntaxKind::RETURN) {
            self.start_node_at(checkpoint, SyntaxKind::ReturnExpression);

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
fn return_expression() {
    for source in ["return 1"] {
        let mut context = crate::Context::new();
        let mut lex = Lex::new(source);
        context.return_expression(&mut lex);
        let node = context.finish();

        assert_eq!(node.to_string(), source);
    }
}
