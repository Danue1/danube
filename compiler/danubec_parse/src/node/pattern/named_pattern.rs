use danubec_lex::Lex;
use danubec_syntax::{Checkpoint, SyntaxKind};

impl crate::Context {
    pub fn named_pattern(&mut self, lex: &mut Lex, checkpoint: Checkpoint) -> bool {
        if expect!(self, lex, SyntaxKind::LEFT_BRACE) {
            self.start_node_at(checkpoint, SyntaxKind::NamedPattern);

            self.trivia(lex);
            while self.named_pattern_element(lex) {
                self.trivia(lex);
                if expect!(self, lex, SyntaxKind::COMMA) {
                    self.trivia(lex);
                } else {
                    break;
                }
            }

            self.trivia(lex);
            expect!(self, lex, SyntaxKind::RIGHT_BRACE);

            self.finish_node();

            true
        } else {
            false
        }
    }

    fn named_pattern_element(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if self.path(lex) {
            self.start_node_at(checkpoint, SyntaxKind::NamedPatternElement);

            self.trivia(lex);
            if expect!(self, lex, SyntaxKind::COLON) {
                self.trivia(lex);
                self.pattern(lex);
            }

            self.finish_node();

            true
        } else {
            false
        }
    }
}

#[test]
fn named_pattern() {
    for source in ["a {}", "a { b: c }", "a { b: c, }", "a { b: c, c: d }"] {
        let mut context = crate::Context::new();
        let mut lex = Lex::new(source);
        context.pattern(&mut lex);
        let node = context.finish();

        assert_eq!(format!("{}", node), source);
    }
}
