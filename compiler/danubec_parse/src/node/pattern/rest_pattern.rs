use danubec_lex::Lex;
use danubec_syntax::SyntaxKind;

impl crate::Context {
    pub fn rest_pattern(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if expect!(self, lex, token -> DOT__DOT, SyntaxKind::DOT, SyntaxKind::DOT,) {
            self.start_node_at(checkpoint, SyntaxKind::RestPattern);
            self.finish_node();

            true
        } else {
            false
        }
    }
}

#[test]
fn rest_pattern() {
    for source in [".."] {
        let mut context = crate::Context::new();
        let mut lex = Lex::new(source);
        context.rest_pattern(&mut lex);
        let node = context.finish();

        assert_eq!(format!("{}", node), source);
    }
}
