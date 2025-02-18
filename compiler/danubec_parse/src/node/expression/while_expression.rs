use danubec_lex::Lex;
use danubec_syntax::SyntaxKind;

impl crate::Context {
    pub fn while_expression(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if expect!(self, lex, SyntaxKind::WHILE) {
            self.start_node_at(checkpoint, SyntaxKind::WhileExpression);

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
fn while_expression() {
    for source in ["while true {}", "while true { if true {} }"] {
        let mut context = crate::Context::new();
        let mut lex = Lex::new(source);
        context.while_expression(&mut lex);
        let node = context.finish();

        assert_eq!(node.to_string(), source);
    }
}
