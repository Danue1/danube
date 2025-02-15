use danubec_lex::Lex;
use danubec_syntax::SyntaxKind;

impl crate::Context {
    pub fn path(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if self.path_segment(lex) {
            self.start_node_at(checkpoint, SyntaxKind::Path);

            self.trivia(lex);
            while expect!(self, lex, token -> COLON__COLON, SyntaxKind::COLON, SyntaxKind::COLON,) {
                self.trivia(lex);
                if !self.path_segment(lex) {
                    break;
                }
            }

            self.finish_node();

            true
        } else {
            false
        }
    }

    fn path_segment(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if self.identifier(lex) {
            self.start_node_at(checkpoint, SyntaxKind::PathSegment);

            self.trivia(lex);
            self.type_argument(lex);

            self.finish_node();

            true
        } else {
            false
        }
    }
}

#[test]
fn path() {
    for source in ["a", "a::b", "a::b<>", "a::b<c>", "a::b<c,>", "a::b<c, d>"] {
        let mut context = crate::Context::new();
        let mut lex = Lex::new(source);
        context.path(&mut lex);
        let node = context.finish();

        assert_eq!(node.to_string(), source);
    }
}
