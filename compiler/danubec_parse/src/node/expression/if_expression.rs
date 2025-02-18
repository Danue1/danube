use danubec_lex::Lex;
use danubec_syntax::SyntaxKind;

impl crate::Context {
    pub fn if_expression(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if expect!(self, lex, SyntaxKind::IF) {
            self.start_node_at(checkpoint, SyntaxKind::IfExpression);

            self.trivia(lex);
            self.expression(lex);

            self.trivia(lex);
            self.block_expression(lex);

            self.trivia(lex);
            let checkpoint = self.checkpoint();
            if expect!(self, lex, SyntaxKind::ELSE) {
                self.start_node_at(checkpoint, SyntaxKind::ElseBranch);

                self.trivia(lex);
                let _ = self.if_expression(lex) || self.block_expression(lex);

                self.finish_node();
            }

            self.finish_node();

            true
        } else {
            false
        }
    }
}

#[test]
fn if_expression() {
    for source in [
        "if true {}",
        "if true {} else {}",
        "if true {} else { if true {} }",
        "if true {} else { if true {} else {} }",
        "if true {} else if true {}",
        "if true {} else if true {} else {}",
        "if true { if true {} }",
        "if true { if true {} else {} }",
    ] {
        let mut context = crate::Context::new();
        let mut lex = Lex::new(source);
        context.if_expression(&mut lex);
        let node = context.finish();

        assert_eq!(node.to_string(), source);
    }
}
