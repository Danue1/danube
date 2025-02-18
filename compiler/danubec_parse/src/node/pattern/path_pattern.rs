use danubec_lex::Lex;
use danubec_syntax::SyntaxKind;

impl crate::Context {
    pub fn path_pattern(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if self.path(lex) {
            let matched =
                self.named_pattern(lex, checkpoint) || self.unnamed_pattern(lex, checkpoint);
            if !matched {
                self.start_node_at(checkpoint, SyntaxKind::PathPattern);
                self.finish_node();
            }

            true
        } else {
            false
        }
    }
}

#[test]
fn path_pattern() {
    for source in ["a", "a::b", "a::b::c"] {
        let mut context = crate::Context::new();
        let mut lex = Lex::new(source);
        context.path_pattern(&mut lex);
        let node = context.finish();

        assert_eq!(format!("{}", node), source);
    }
}
