use danubec_lex::Lex;
use danubec_syntax::SyntaxKind;

impl crate::Context {
    pub fn unary_expression(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if expect!(
            self,
            lex,
            node ->
            UnaryOperator,
            SyntaxKind::HYPHEN | SyntaxKind::EXCLAMATION | SyntaxKind::TILDE,
        ) {
            self.start_node_at(checkpoint, SyntaxKind::UnaryExpression);
            self.expression(lex);
            self.finish_node();

            true
        } else {
            false
        }
    }
}

#[test]
fn unary_expression() {
    for source in ["-1", "!true", "~1"] {
        let mut context = crate::Context::new();
        let mut lex = Lex::new(source);
        context.unary_expression(&mut lex);
        let node = context.finish();

        assert_eq!(format!("{}", node), source);
    }
}
