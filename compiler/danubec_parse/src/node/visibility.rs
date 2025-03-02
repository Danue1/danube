use danubec_lex::Lex;
use danubec_syntax::SyntaxKind;

impl crate::Context {
    pub fn visibility(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if expect!(self, lex, SyntaxKind::PUB) {
            self.start_node_at(checkpoint, SyntaxKind::Visibility);

            if expect!(self, lex, SyntaxKind::LEFT_PAREN) {
                self.trivia(lex);

                let checkpoint = self.checkpoint();
                if expect!(self, lex, SyntaxKind::CRATE) {
                    self.start_node_at(checkpoint, SyntaxKind::VisibilityCrate);
                    self.finish_node();
                } else if expect!(self, lex, SyntaxKind::SUPER) {
                    self.start_node_at(checkpoint, SyntaxKind::VisibilitySuper);
                    self.finish_node();
                } else if expect!(self, lex, SyntaxKind::IN) {
                    self.start_node_at(checkpoint, SyntaxKind::VisibilityIn);
                    self.trivia(lex);
                    self.path(lex);
                    self.finish_node();
                }

                self.trivia(lex);
                expect!(self, lex, SyntaxKind::RIGHT_PAREN);
            }

            self.finish_node();

            true
        } else {
            false
        }
    }
}

#[test]
fn visibility() {
    for source in ["pub", "pub(crate)", "pub(super)", "pub(in something)"] {
        let mut context = crate::Context::new();
        let mut lex = Lex::new(source);
        context.visibility(&mut lex);
        let node = context.finish();

        assert_eq!(node.kind(), SyntaxKind::Visibility.into());
        assert_eq!(format!("{}", node), source);
    }
}
