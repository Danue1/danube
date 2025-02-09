use danubec_lex::Lex;
use danubec_syntax::SyntaxKind;

impl crate::Context {
    pub fn char_literal(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if expect!(self, lex, SyntaxKind::SINGLE_QUOTE) {
            self.start_node_at(checkpoint, SyntaxKind::CharLiteral);

            expect!(self, lex, SyntaxKind::BACKSLASH);
            if let Some((kind, source)) = lex.next() {
                self.start_node(SyntaxKind::Raw);
                self.token(kind, &source[..1]);
                if source.len() > 1 {
                    self.start_node(SyntaxKind::Error);
                    self.token(kind, &source[1..]);
                    self.finish_node();
                }
                self.finish_node();
            }

            let checkpoint = self.checkpoint();
            if !expect!(self, lex, SyntaxKind::SINGLE_QUOTE) {
                self.start_node_at(checkpoint, SyntaxKind::Error);
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
fn char_literal() {
    for source in ["'a'", "'\\a'"] {
        let mut context = crate::Context::new();
        let mut lex = Lex::new(source);
        context.char_literal(&mut lex);
        let node = context.finish();

        assert_eq!(format!("{}", node), source);
    }
}
