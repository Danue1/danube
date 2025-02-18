use crate::pratt::Bp;
use danubec_lex::Lex;
use danubec_syntax::{Checkpoint, SyntaxKind};

impl crate::Context {
    pub fn or_pattern(&mut self, lex: &mut Lex, checkpoint: Checkpoint, binding_power: Bp) -> bool {
        if expect!(self, lex, SyntaxKind::PIPE) {
            self.start_node_at(checkpoint, SyntaxKind::OrPattern);

            self.trivia(lex);
            let checkpoint = self.checkpoint();
            if self.pattern_kind_bp(lex, binding_power) {
                self.start_node_at(checkpoint, SyntaxKind::OrPatternRhs);
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
fn or_pattern() {
    for source in ["a", "a | b", "a | b | c"] {
        let mut context = crate::Context::new();
        let mut lex = Lex::new(source);
        context.pattern(&mut lex);
        let node = context.finish();

        assert_eq!(format!("{}", node), source);
    }
}
