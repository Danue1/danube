use danubec_lex::Lex;
use danubec_syntax::SyntaxKind;

impl crate::Context {
    pub fn literal_pattern(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if self.literal(lex) {
            self.start_node_at(checkpoint, SyntaxKind::LiteralPattern);
            self.finish_node();

            true
        } else {
            false
        }
    }
}

#[test]
fn literal_pattern() {
    for source in ["1", "1.0", "true", "false", "'a'", "\"a\""] {
        let mut context = crate::Context::new();
        let mut lex = Lex::new(source);
        context.literal_pattern(&mut lex);
        let node = context.finish();

        assert_eq!(format!("{}", node), source);
    }
}
