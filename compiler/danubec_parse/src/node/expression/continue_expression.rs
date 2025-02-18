use danubec_lex::Lex;
use danubec_syntax::SyntaxKind;

impl crate::Context {
    pub fn continue_expression(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if expect!(self, lex, SyntaxKind::CONTINUE) {
            self.start_node_at(checkpoint, SyntaxKind::ContinueExpression);
            self.finish_node();

            true
        } else {
            false
        }
    }
}

#[test]
fn continue_expression() {
    for source in ["continue"] {
        let mut context = crate::Context::new();
        let mut lex = Lex::new(source);
        context.continue_expression(&mut lex);
        let node = context.finish();

        assert_eq!(node.to_string(), source);
    }
}
