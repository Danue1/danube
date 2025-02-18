use danubec_lex::Lex;
use danubec_syntax::{Checkpoint, SyntaxKind};

impl crate::Context {
    pub fn unnamed_pattern(&mut self, lex: &mut Lex, checkpoint: Checkpoint) -> bool {
        if expect!(self, lex, SyntaxKind::LEFT_PAREN) {
            self.start_node_at(checkpoint, SyntaxKind::UnnamedPattern);

            self.trivia(lex);
            while self.unnamed_pattern_element(lex) {
                self.trivia(lex);
                if expect!(self, lex, SyntaxKind::COMMA) {
                    self.trivia(lex);
                } else {
                    break;
                }
            }

            self.trivia(lex);
            expect!(self, lex, SyntaxKind::RIGHT_PAREN);

            self.finish_node();

            true
        } else {
            false
        }
    }

    fn unnamed_pattern_element(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if self.pattern(lex) {
            self.start_node_at(checkpoint, SyntaxKind::UnnamedPatternElement);
            self.finish_node();

            true
        } else {
            false
        }
    }
}

#[test]
fn unnamed_pattern() {
    for source in ["a()", "a(b)", "a(b,)", "a(b, c)"] {
        let mut context = crate::Context::new();
        let mut lex = Lex::new(source);
        context.pattern(&mut lex);
        let node = context.finish();

        assert_eq!(format!("{}", node), source);
    }
}
