use danubec_lex::Lex;
use danubec_syntax::SyntaxKind;

impl crate::Context {
    pub fn path_type(&mut self, lex: &mut Lex) -> bool {
        let checkpoint = self.checkpoint();
        if self.identifier(lex) {
            self.start_node_at(checkpoint, SyntaxKind::PathType);

            self.trivia(lex);

            while expect!(
                self,
                lex,
                COLON__COLON,
                SyntaxKind::COLON,
                SyntaxKind::COLON,
            ) {
                self.trivia(lex);
                if self.identifier(lex) {
                    self.trivia(lex);
                } else {
                    break;
                }
            }

            self.finish_node();

            true
        } else {
            false
        }
    }
}

#[test]
fn path_type() {
    for source in ["foo", "foo::bar", "foo::bar::baz"] {
        let mut context = crate::Context::new();
        let mut lex = Lex::new(source);
        context.path_type(&mut lex);
        let node = context.finish();

        assert_eq!(node.kind(), SyntaxKind::PathType);
        assert_eq!(format!("{}", node), source);
    }
}
