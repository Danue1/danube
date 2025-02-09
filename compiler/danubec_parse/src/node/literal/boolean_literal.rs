use danubec_lex::Lex;
use danubec_syntax::SyntaxKind;

impl crate::Context {
    pub fn boolean_literal(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if expect!(self, lex, SyntaxKind::FALSE) || expect!(self, lex, SyntaxKind::TRUE) {
            self.start_node_at(checkpoint, SyntaxKind::BooleanLiteral);
            self.finish_node();

            true
        } else {
            false
        }
    }
}

#[test]
fn boolean_literal() {
    for source in ["true", "false"] {
        let mut context = crate::Context::new();
        let mut lex = Lex::new(source);
        context.boolean_literal(&mut lex);
        let node = context.finish();

        assert_eq!(format!("{}", node), source);
    }
}
