use danubec_lex::Lex;
use danubec_syntax::SyntaxKind;

impl crate::Context {
    pub fn break_expression(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if expect!(self, lex, SyntaxKind::BREAK) {
            self.start_node_at(checkpoint, SyntaxKind::BreakExpression);
            self.finish_node();

            true
        } else {
            false
        }
    }
}

#[test]
fn break_expression() {
    for source in ["break"] {
        let mut context = crate::Context::new();
        let mut lex = Lex::new(source);
        context.break_expression(&mut lex);
        let node = context.finish();

        assert_eq!(node.to_string(), source);
    }
}
