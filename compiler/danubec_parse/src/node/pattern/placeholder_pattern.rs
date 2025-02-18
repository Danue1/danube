use danubec_lex::Lex;
use danubec_syntax::SyntaxKind;

impl crate::Context {
    pub fn placeholder_pattern(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if expect!(self, lex, SyntaxKind::UNDERSCORE) {
            self.start_node_at(checkpoint, SyntaxKind::PlaceholderPattern);
            self.finish_node();

            true
        } else {
            false
        }
    }
}

#[test]
fn placeholder_pattern() {
    for source in ["_"] {
        let mut context = crate::Context::new();
        let mut lex = Lex::new(source);
        context.placeholder_pattern(&mut lex);
        let node = context.finish();

        assert_eq!(format!("{}", node), source);
    }
}
