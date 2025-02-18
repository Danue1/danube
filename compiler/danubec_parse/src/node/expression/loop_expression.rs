use danubec_lex::Lex;
use danubec_syntax::SyntaxKind;

impl crate::Context {
    pub fn loop_expression(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if expect!(self, lex, SyntaxKind::LOOP) {
            self.start_node_at(checkpoint, SyntaxKind::LoopExpression);

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
fn loop_expression() {
    for source in ["loop {}", "loop { loop {} }", "loop { loop {} loop {} }"] {
        let mut context = crate::Context::new();
        let mut lex = Lex::new(source);
        context.loop_expression(&mut lex);
        let node = context.finish();

        assert_eq!(node.to_string(), source);
    }
}
