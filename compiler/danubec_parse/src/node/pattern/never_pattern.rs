use danubec_lex::Lex;
use danubec_syntax::SyntaxKind;

impl crate::Context {
    pub fn never_pattern(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if expect!(self, lex, SyntaxKind::EXCLAMATION) {
            self.start_node_at(checkpoint, SyntaxKind::NeverPattern);
            self.finish_node();

            true
        } else {
            false
        }
    }
}

#[test]
fn never_pattern() {
    for source in ["!"] {
        let mut context = crate::Context::new();
        let mut lex = Lex::new(source);
        context.never_pattern(&mut lex);
        let node = context.finish();

        assert_eq!(format!("{}", node), source);
    }
}
